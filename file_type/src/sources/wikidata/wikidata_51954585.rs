use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51954585: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_585,
        source_type: SourceType::Wikidata,
        name: "dBASE III+ file format",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
