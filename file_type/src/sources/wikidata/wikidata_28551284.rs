use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
