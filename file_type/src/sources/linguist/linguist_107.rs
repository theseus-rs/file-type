use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_107: FileFormat = FileFormat {
    id: 107,
    source_type: SourceType::Linguist,
    name: "Fortran",
    extensions: &["f", "f77", "for", "fpp"],
    media_types: &["text/x-fortran"],
    internal_signatures: &[],
    related_formats: &[],
};
