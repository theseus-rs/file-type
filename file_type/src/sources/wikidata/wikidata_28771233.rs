use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28771233: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_233,
        source_type: SourceType::Wikidata,
        name: "MINC",
        extensions: &["mnc"],
        media_types: &["application/x-minc"],
        signatures: &[],
        related_formats: &[],
    },
};
