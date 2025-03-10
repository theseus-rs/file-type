use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3502441: FileType = FileType {
    file_format: &FileFormat {
        id: 3_502_441,
        source_type: SourceType::Wikidata,
        name: "Excel Binary File Format",
        extensions: &[],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
