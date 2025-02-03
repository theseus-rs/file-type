use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_374: FileFormat = FileFormat {
    id: 374,
    source_type: SourceType::Linguist,
    name: "Thrift",
    extensions: &["thrift"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
