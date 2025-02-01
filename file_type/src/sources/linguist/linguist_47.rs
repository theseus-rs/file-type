use crate::format::FileFormat;

pub(crate) const LINGUIST_47: FileFormat = FileFormat {
    id: 47,
    puid: "linguist/47",
    name: "CMake",
    extensions: &["cmake", "cmake.in"],
    media_types: &["text/x-cmake"],
    internal_signatures: &[],
    related_formats: &[],
};
