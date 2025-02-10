use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967381: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_381,
        source_type: SourceType::Wikidata,
        name: "Gravis Ultrasound patch",
        extensions: &["pat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
