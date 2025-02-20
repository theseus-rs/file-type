use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66305603: FileType = FileType {
    file_format: &FileFormat {
        id: 66_305_603,
        source_type: SourceType::Wikidata,
        name: "Data Source Name file format",
        extensions: &["dsn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
