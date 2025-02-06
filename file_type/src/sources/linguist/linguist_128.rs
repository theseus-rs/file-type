use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_128: FileFormat = FileFormat {
    id: 128,
    source_type: SourceType::Linguist,
    name: "Gentoo Eclass",
    extensions: &["eclass"],
    media_types: &["text/x-sh"],
    signatures: &[],
    related_formats: &[],
};
