use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133843426: FileType = FileType {
    file_format: &FileFormat {
        id: 133_843_426,
        source_type: SourceType::Wikidata,
        name: "Saracen Paint file",
        extensions: &["sar"],
        media_types: &["image/x-saracen-paint"],
        signatures: &[],
        related_formats: &[],
    },
};
