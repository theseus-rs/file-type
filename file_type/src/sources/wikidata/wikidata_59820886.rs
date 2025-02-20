use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59820886: FileType = FileType {
    file_format: &FileFormat {
        id: 59_820_886,
        source_type: SourceType::Wikidata,
        name: "Corel CMX Compressed",
        extensions: &["cpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
