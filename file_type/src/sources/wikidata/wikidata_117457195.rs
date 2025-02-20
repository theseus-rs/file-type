use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117457195: FileType = FileType {
    file_format: &FileFormat {
        id: 117_457_195,
        source_type: SourceType::Wikidata,
        name: "Raw PIMA SWIR Reflectance Spectral File",
        extensions: &["fos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
