use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_969674868: FileFormat = FileFormat {
    id: 969_674_868,
    source_type: SourceType::Linguist,
    name: "Windows Registry Entries",
    extensions: &["reg"],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
