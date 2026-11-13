#!/usr/bin/env python3
import gi
gi.require_version('IBus', '1.0')
from gi.repository import IBus, GLib
import sys

# বিশেষ ও জটিল যুক্তবর্ণ (Special Conjuncts & Shortcuts)
COMPOUNDS = {
    'ksh': 'ক্ষ', 'kkh': 'ক্ষ', 'x': 'ক্স',
    'jng': 'জ্ঞ', 'gg': 'জ্ঞ', 'gG': 'জ্ঞ',
    'ngk': 'ঙ্ক', 'ngkh': 'ঙ্খ', 'ngg': 'ঙ্গ', 'nggh': 'ঙ্ঘ',
    'nch': 'ঞ্চ', 'nc': 'ঞ্চ', 'nchh': 'ঞ্ছ', 'nj': 'ঞ্জ', 'njh': 'ঞ্ঝ',
    'tt': 'ত্ত', 'tth': 'ত্থ', 'tn': 'ত্ন', 'tm': 'ত্ম', 'tw': 'ত্ব',
    'dd': 'দ্দ', 'ddh': 'দ্ধ', 'dw': 'দ্ব', 'dm': 'দ্ম',
    'nt': 'ন্ত', 'nth': 'ন্থ', 'nd': 'ন্দ', 'ndh': 'ন্ধ', 'nn': 'ন্ন', 'nw': 'ন্ব', 'nm': 'ন্ম',
    'mp': 'ম্প', 'mph': 'ম্ফ', 'mb': 'ম্ব', 'mbh': 'ম্ভ', 'mm': 'ম্ম', 'ml': 'ম্ল',
    'lk': 'ল্ক', 'lg': 'ল্গ', 'lt': 'ল্ট', 'ld': 'ল্ড', 'lp': 'ল্প', 'lm': 'ল্ম', 'll': 'ল্ল',
    'shc': 'শ্চ', 'shch': 'শ্ছ', 'shn': 'শ্ন', 'shw': 'শ্ব', 'shm': 'শ্ম', 'shr': 'শ্র', 'shl': 'শ্ল',
    'Sk': 'ষ্ক', 'Skr': 'ষ্ক্র', 'ShT': 'ষ্ট', 'ShTh': 'ষ্ঠ', 'ShN': 'ষ্ণ', 'Sp': 'ষ্প', 'Sph': 'ষ্ফ', 'Sm': 'ষ্ম',
    'sk': 'স্ক', 'skr': 'স্ক্র', 'skh': 'স্খ', 'st': 'স্ত', 'str': 'স্ত্র', 'sth': 'স্থ', 'sn': 'স্ন',
    'sp': 'স্প', 'sph': 'স্ফ', 'sw': 'স্ব', 'sm': 'স্ম', 'sl': 'স্ল',
    'hm': 'হ্ম', 'hn': 'হ্ণ', 'hl': 'হ্ল', 'hr': 'হ্র', 'hw': 'হ্ব',
    'tr': 'ত্র', 'dr': 'দ্র', 'dhr': 'ধ্র', 'pr': 'প্র', 'br': 'ব্র', 'bhr': 'ভ্র', 'mr': 'ম্র',
    'bd': 'ব্দ', 'bdh': 'ব্ধ', 'bj': 'ব্জ', 'bb': 'ব্ব', 'bl': 'ব্ল',
    'kt': 'ক্ত', 'ktr': 'ক্ত্র', 'kr': 'ক্র', 'kl': 'ক্ল', 'kw': 'ক্ব', 'kk': 'ক্ক',
    'gd': 'গ্ধ', 'gn': 'গ্ন', 'gr': 'গ্র', 'gl': 'গ্ল', 'gw': 'গ্ব',
    'rri': 'ঋ'
}

CONSONANTS = {
    'k': 'ক', 'kh': 'খ', 'g': 'গ', 'gh': 'ঘ', 'Ng': 'ঙ', 'ng': 'ং',
    'c': 'চ', 'ch': 'ছ', 'j': 'জ', 'jh': 'ঝ', 'NG': 'ঞ',
    'T': 'ট', 'Th': 'ঠ', 'D': 'ড', 'Dh': 'ঢ', 'N': 'ণ',
    't': 'ত', 'th': 'থ', 'd': 'দ', 'dh': 'ধ', 'n': 'ন',
    'p': 'প', 'ph': 'ফ', 'f': 'ফ', 'b': 'ব', 'bh': 'ভ', 'v': 'ভ', 'm': 'ম',
    'z': 'য', 'y': 'য়', 'r': 'র', 'l': 'ল',
    'sh': 'শ', 'S': 'ষ', 's': 'স', 'h': 'হ',
    'R': 'ড়', 'Rh': 'ঢ়', 'Y': 'য়', 't``': 'ৎ', 'H': 'ঃ', '^': 'ঁ'
}

VOWELS = {
    'o': 'অ', 'a': 'আ', 'i': 'ই', 'I': 'ঈ', 'ee': 'ঈ',
    'u': 'উ', 'U': 'ঊ', 'oo': 'ঊ', 'rri': 'ঋ',
    'e': 'এ', 'OI': 'ঐ', 'O': 'ও', 'OU': 'ঔ'
}

KAR_SIGNS = {
    'o': '', 'a': 'া', 'i': 'ি', 'I': 'ী', 'ee': 'ী',
    'u': 'ু', 'U': 'ূ', 'oo': 'ূ', 'rri': 'ৃ',
    'e': 'ে', 'OI': 'ৈ', 'O': 'ো', 'OU': 'ৌ'
}

NUMBERS = {'0':'০','1':'১','2':'২','3':'৩','4':'৪','5':'৫','6':'৬','7':'৭','8':'৮','9':'৯'}

def parse_avro(text):
    res = []
    i = 0
    n = len(text)
    prev_type = 'none'
    all_keys = list(COMPOUNDS.keys()) + list(CONSONANTS.keys()) + list(VOWELS.keys())
    sorted_keys = sorted(set(all_keys), key=len, reverse=True)
    
    while i < n:
        if text[i] in NUMBERS:
            res.append(NUMBERS[text[i]])
            prev_type = 'num'
            i += 1
            continue
        if text[i:i+2] == ',,':
            res.append('্')
            prev_type = 'hasanta'
            i += 2
            continue
        if text[i:i+3] == 't``':
            res.append('ৎ')
            prev_type = 'other'
            i += 3
            continue
            
        matched = False
        for k in sorted_keys:
            if text.startswith(k, i):
                if k == 'rri':
                    if prev_type == 'cons':
                        res.append('ৃ')
                    else:
                        res.append('ঋ')
                    prev_type = 'vowel'
                elif k in COMPOUNDS:
                    if prev_type == 'cons':
                        res.append('্' + COMPOUNDS[k])
                    else:
                        res.append(COMPOUNDS[k])
                    prev_type = 'cons'
                elif k in CONSONANTS:
                    if prev_type == 'cons':
                        if k == 'y':
                            res.append('্য')
                        elif k == 'w':
                            res.append('্ব')
                        elif k == 'r' and (not res or res[-1] != 'র'):
                            res.append('্র')
                        elif res and res[-1] == 'র' and k != 'y':
                            res[-1] = 'র্' + CONSONANTS[k]
                        else:
                            res.append('্' + CONSONANTS[k])
                    else:
                        res.append(CONSONANTS[k])
                    prev_type = 'cons'
                elif k in VOWELS:
                    if prev_type == 'cons':
                        res.append(KAR_SIGNS[k])
                    else:
                        res.append(VOWELS[k])
                    prev_type = 'vowel'
                i += len(k)
                matched = True
                break
        if not matched:
            res.append(text[i])
            prev_type = 'other'
            i += 1
    return ''.join(res)

class AvroEngine(IBus.Engine):
    def __init__(self):
        super().__init__()
        self.buffer = ""
        self.lookup_table = IBus.LookupTable.new(5, 0, True, True)

    def do_process_key_event(self, keyval, keycode, state):
        if state & IBus.ModifierType.RELEASE_MASK:
            return False
        if state & (IBus.ModifierType.CONTROL_MASK | IBus.ModifierType.MOD1_MASK):
            return False

        if keyval == IBus.KEY_BackSpace:
            if self.buffer:
                self.buffer = self.buffer[:-1]
                self._update()
                return True
            return False

        if keyval in (IBus.KEY_space, IBus.KEY_Return, IBus.KEY_Tab):
            if self.buffer:
                self._commit()
            return False

        if keyval == IBus.KEY_Escape:
            if self.buffer:
                self.buffer = ""
                self._update()
                return True
            return False

        if 33 <= keyval <= 126:
            self.buffer += chr(keyval)
            self._update()
            return True

        return False

    def _update(self):
        if not self.buffer:
            self.hide_preedit_text()
            self.hide_lookup_table()
            return
        converted = parse_avro(self.buffer)
        text = IBus.Text.new_from_string(converted)
        self.update_preedit_text(text, len(converted), True)
        self.lookup_table.clear()
        self.lookup_table.append_candidate(IBus.Text.new_from_string(converted))
        self.lookup_table.append_candidate(IBus.Text.new_from_string(self.buffer))
        self.update_lookup_table(self.lookup_table, True)

    def _commit(self):
        converted = parse_avro(self.buffer)
        self.commit_text(IBus.Text.new_from_string(converted))
        self.buffer = ""
        self.hide_preedit_text()
        self.hide_lookup_table()

    def do_focus_out(self):
        if self.buffer:
            self._commit()

class IMApp:
    def __init__(self, exec_by_ibus):
        IBus.init()
        self.bus = IBus.Bus()
        self.factory = IBus.Factory.new(self.bus.get_connection())
        self.factory.add_engine("avro-fedora", AvroEngine)
        if exec_by_ibus:
            self.bus.request_name("org.freedesktop.IBus.AvroFedora", 0)
        else:
            comp = IBus.Component.new("org.freedesktop.IBus.AvroFedora", "Universal Linux Avro Phonetic", "1.0", "GPL", "Sabbir", "", "", "avro-fedora")
            engine = IBus.EngineDesc.new("avro-fedora", "Avro Phonetic (Custom)", "Bangla Phonetic Keyboard", "bn", "GPL", "Sabbir", "", "us")
            comp.add_engine(engine)
            self.bus.register_component(comp)
            self.bus.set_global_engine_async("avro-fedora", -1, None, None, None)

    def run(self):
        GLib.MainLoop().run()

if __name__ == "__main__":
    IMApp("--ibus" in sys.argv).run()
