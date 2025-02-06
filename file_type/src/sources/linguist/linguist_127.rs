use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_127: FileFormat = FileFormat {
    id: 127,
    source_type: SourceType::Linguist,
    name: "Gentoo Ebuild",
    extensions: &["ebuild"],
    media_types: &["text/x-sh"],
    signatures: &[],
    related_formats: &[],
};
