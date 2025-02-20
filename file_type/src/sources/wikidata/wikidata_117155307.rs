use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117155307: FileType = FileType {
    file_format: &FileFormat {
        id: 117_155_307,
        source_type: SourceType::Wikidata,
        name: "Picture It! PNG Plus",
        extensions: &["png"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
