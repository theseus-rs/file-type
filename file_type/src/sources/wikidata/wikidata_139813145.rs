use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139813145: FileType = FileType {
    file_format: &FileFormat {
        id: 139_813_145,
        source_type: SourceType::Wikidata,
        name: "Text Converter",
        extensions: &["wpc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
