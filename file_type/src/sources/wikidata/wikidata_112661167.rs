use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112661167: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_167,
        source_type: SourceType::Wikidata,
        name: "Motion Analysis HTR File",
        extensions: &["htr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
