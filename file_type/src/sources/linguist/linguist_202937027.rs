use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_202937027: FileFormat = FileFormat {
    id: 202_937_027,
    source_type: SourceType::Linguist,
    name: "Motoko",
    extensions: &["mo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
