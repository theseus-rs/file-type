use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009215: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_215,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Banner File Format",
        extensions: &["ban"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
