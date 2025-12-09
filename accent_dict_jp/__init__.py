import os.path
import random
import string
from aqt import mw
from aqt.qt import QMenuBar, QFont, QMenu, QWidget, QVBoxLayout, QAction, Qt
from aqt.editor import Editor
from aqt import gui_hooks
from typing import Optional
from anki.notes import Note
from os import getcwd
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

        print("addon_path: " + getcwd())

    def set_field(self, field_name: str, val: str) -> None:
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
    
    def get_field(self, field_name: str) -> Optional[str]:
        note = self.editor.note
        if note is None:
            return None
        tp = note.note_type()
        if tp is None:
            return None
        for idx, field in enumerate(tp["flds"]):
            if field["name"] == field_name:
                return note.fields[idx]
        return None
    

    def save_audio(self, sound_file: str) -> None:
        raw = get_sound(os.path.join(os.path.dirname(os.path.normpath(__file__)), "assets"), sound_file)
        if mw is None:
            return None
        if mw.col is None:
            return None
        mw.col.media.write_data(sound_file, bytes(raw))

    def save_pitch(self, pitch: str) -> None:
        pitch_svg = gen_pitch_svg(pitch)
        if mw is None:
            return None
        if mw.col is None:
            return None
        mw.col.media.write_data(sanitise_str(pitch) + ".svg", str.encode(pitch_svg))

    def write_voc(self, id: str, sound_file: Optional[str], pitch: str, voc: str) -> None:
        self.set_field("dict", id)
        self.set_field("voc", voc)
        self.set_field("pitch", '<img src="' + sanitise_str(pitch) + '.svg">')
        self.save_pitch(pitch)

        if sound_file is not None:
            self.set_field("audio", "[sound:" + sound_file + "]")
            self.save_audio(sound_file)

    def regenerated_actions(self):
        self.dict_menu.clear()
        vocab_str = self.get_field("dict")
        vocabs = []
        if vocab_str is not None:
            vocabs = look_up(os.path.join(os.path.dirname(os.path.normpath(__file__)), "assets"), vocab_str)
        for vocab in vocabs:
            vocab_menu = QMenu(vocab.head, self.editor.parentWindow)

            for pron in vocab.pron:
                pron_action = QAction(pron.accent, self.editor.parentWindow)
                pron_action.setFont(self.font)
                pron_action.triggered.connect(
                    lambda _, id=vocab.id + '_' + pron.id, sound_file=pron.sound_file, pitch=pron.accent,
                           voc=vocab.head: self.write_voc(id, sound_file, pitch, voc))
                vocab_menu.addAction(pron_action)

            self.dict_menu.addMenu(vocab_menu)


def create_dict(editor: Editor) -> None:
    editor_dictionary_instance[editor] = Dictionary(editor)

gui_hooks.editor_did_init.append(create_dict)