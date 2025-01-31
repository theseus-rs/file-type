use crate::format::FileFormat;

pub(crate) const LINGUIST_70: FileFormat = FileFormat {
    id: 70,
    puid: "linguist/70",
    name: "Cpp-ObjDump",
    extensions: &[
        "c++-objdump",
        "c++objdump",
        "cpp-objdump",
        "cppobjdump",
        "cxx-objdump",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
