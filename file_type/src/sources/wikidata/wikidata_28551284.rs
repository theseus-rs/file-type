use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551284: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_284,
        source_type: SourceType::Wikidata,
        name: "Adobe CMYK Setup File",
        extensions: &["api"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
