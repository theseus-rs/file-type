use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50563011: FileType = FileType {
    file_format: &FileFormat {
        id: 50_563_011,
        source_type: SourceType::Wikidata,
        name: "BKNAS Seismic Data Format",
        extensions: &["bknas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
