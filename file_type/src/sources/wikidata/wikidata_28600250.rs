use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600250: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_250,
        source_type: SourceType::Wikidata,
        name: "Oracle database backup format",
        extensions: &["arc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
