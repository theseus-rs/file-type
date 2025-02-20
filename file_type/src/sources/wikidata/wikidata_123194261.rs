use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
