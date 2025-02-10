use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83884461: FileType = FileType {
    file_format: &FileFormat {
        id: 83_884_461,
        source_type: SourceType::Wikidata,
        name: "Windows Address Book",
        extensions: &["wab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
