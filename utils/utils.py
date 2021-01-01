class RoundList(list):
    def __getitem__(self, index):
        while index not in range(0, len(self)):
            if index >= len(self):
                index = index - len(self)
            else:
                if index < 0: index = len(self) - (index * -1)
        return list.__getitem__(self, index)


def default_dict(keygen):
    global _DEFAULT_DICT_ID
    default_dict_class_name = 'DefaultDict' + str(_DEFAULT_DICT_ID)
    _DEFAULT_DICT_ID += 1

    default_dict_class = type(
        default_dict_class_name, (dict,), {'__missing__': keygen, })
    return default_dict_class()


_DEFAULT_DICT_ID = 0
