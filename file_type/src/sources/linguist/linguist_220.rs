use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_220: FileFormat = FileFormat {
    id: 220,
    source_type: SourceType::Linguist,
    name: "Makefile",
    extensions: &["d", "mak", "make", "makefile", "mk", "mkfile"],
    media_types: &["text/x-cmake"],
    internal_signatures: &[],
    related_formats: &[],
};
