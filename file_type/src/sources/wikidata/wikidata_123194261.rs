use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123194261: FileType = FileType {
    file_format: &FileFormat {
        id: 123_194_261,
        source_type: SourceType::Wikidata,
        name: "Comodo Backup File",
        extensions: &["cbu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
