use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127699802: FileType = FileType {
    file_format: &FileFormat {
        id: 127_699_802,
        source_type: SourceType::Wikidata,
        name: "Futhark file",
        extensions: &["fut"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
