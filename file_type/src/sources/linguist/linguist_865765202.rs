use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_865765202: FileFormat = FileFormat {
    id: 865_765_202,
    source_type: SourceType::Linguist,
    name: "Record Jar",
    extensions: &[],
    media_types: &["text/x-properties"],
    signatures: &[],
    related_formats: &[],
};
