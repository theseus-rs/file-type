use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122673484: FileType = FileType {
    file_format: &FileFormat {
        id: 122_673_484,
        source_type: SourceType::Wikidata,
        name: "Outline 4D Document",
        extensions: &["syv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
