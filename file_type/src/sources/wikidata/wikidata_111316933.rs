use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111316933: FileType = FileType {
    file_format: &FileFormat {
        id: 111_316_933,
        source_type: SourceType::Wikidata,
        name: "Kurzweil K2600 file",
        extensions: &["k26"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
