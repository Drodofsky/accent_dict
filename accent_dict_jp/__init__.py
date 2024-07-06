import os.path
import random
import string
from aqt import mw
from aqt.qt import *
from aqt.editor import Editor
from aqt import gui_hooks
from typing import Optional
from anki.notes import Note
from os import *
import base64
from accent_dict import look_up, get_sound, gen_pitch_svg  # type: ignore
editor_dictionary_instance = {}


def sanitise_str(s: str) -> str:
    return base64.b32encode(str.encode(s)).decode('utf-8').rstrip('=')


def generate_random_string(length: int) -> str:
    # Define the characters to choose from
    characters = string.ascii_letters + string.digits + string.punctuation
    # Generate the random string
    random_string = ''.join(random.choice(characters) for _ in range(length))
    return random_string


class Dictionary:
    def __init__(self, editor: Editor):
        # init ui
        self.editor = editor
        self.menubar = QMenuBar(editor.widget)
        self.font = QFont()
        self.font.setPointSize(20)
        self.menubar.setFont(self.font)
        self.dict_menu = QMenu("Dict", self.menubar)
        self.dict_menu.setFont(self.font)
        self.dict_menu.aboutToShow.connect(self.regenerated_actions)
        self.menubar.addMenu(self.dict_menu)

        self.container_widget = QWidget(editor.widget)
        self.container_layout = QVBoxLayout()
        self.container_layout.addWidget(self.menubar, alignment=Qt.AlignmentFlag.AlignCenter)
        self.container_widget.setLayout(self.container_layout)
        self.editor.outerLayout.insertWidget(0, self.container_widget)

        # init dict
        self.vocab = ""
        print("addon_path: " + getcwd())

    def update_vocab(self, note: Note) -> None:
        tp = note.note_type()
        if tp is None:
            return None
        for idx, field in enumerate(tp["flds"]):
            if field["name"] == "dict":
                front_content = note.fields[idx]
                if self.vocab != front_content:
                    self.vocab = front_content
                    print(self.vocab)

    def update_field(self, field_name: str, val: str) -> None:
        note = self.editor.note
        if note is None:
            return None
        tp = note.note_type()
        if tp is None:
            return None
        for idx, field in enumerate(tp["flds"]):
            if field["name"] == field_name:
                note.fields[idx] = val
                self.editor.set_note(note)
                self.editor.loadNote()

    def save_audio(self, sound_file: str) -> None:
        raw = get_sound(os.path.join(os.path.dirname(os.path.normpath(__file__)), "assets"), sound_file)
        if mw is None:
            return None
        mw.col.media.write_data(sound_file, bytes(raw))

    def save_pitch(self, pitch: str) -> None:
        pitch_svg = gen_pitch_svg(pitch)
        if mw is None:
            return None
        mw.col.media.write_data(sanitise_str(pitch) + ".svg", str.encode(pitch_svg))

    def write_voc(self, id: str, sound_file: str, pitch: str, kanji: Optional[str]) -> None:
        self.update_field("dict", id)
        if kanji is not None:
            self.update_field("kanji", kanji)
        self.update_field("audio", "[sound:" + sound_file + "]")
        self.update_field("pitch", '<img src="' + sanitise_str(pitch) + '.svg">')
        self.save_audio(sound_file)
        self.save_pitch(pitch)

    def regenerated_actions(self):
        self.dict_menu.clear()
        vocabs = look_up(os.path.join(os.path.dirname(os.path.normpath(__file__)), "assets"), self.vocab)
        for vocab in vocabs:
            vocab_menu = QMenu(vocab.head, self.editor.parentWindow)

            for pron in vocab.pron:
                pron_action = QAction(pron.accent, self.editor.parentWindow)
                pron_action.setFont(self.font)
                pron_action.triggered.connect(
                    lambda _, id=vocab.id, sound_file=pron.sound_file, pitch=pron.accent,
                           kanji=vocab.kanji: self.write_voc(id, sound_file, pitch, kanji))
                vocab_menu.addAction(pron_action)

            self.dict_menu.addMenu(vocab_menu)


def create_dict(editor: Editor) -> None:
    editor_dictionary_instance[editor] = Dictionary(editor)


def on_text_update(note: Note) -> None:
    for dict in editor_dictionary_instance.values():
        dict.update_vocab(note)


gui_hooks.editor_did_init.append(create_dict)
gui_hooks.editor_did_fire_typing_timer.append(on_text_update)
