use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1269709: FileType = FileType {
    file_format: &FileFormat {
        id: 1_269_709,
        source_type: SourceType::Wikidata,
        name: "PHP Archive",
        extensions: &["phar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
