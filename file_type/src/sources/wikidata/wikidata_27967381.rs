use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
