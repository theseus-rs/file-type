use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61964300: FileType = FileType {
    file_format: &FileFormat {
        id: 61_964_300,
        source_type: SourceType::Wikidata,
        name: "GSSI SIR-10 RADAN data file",
        extensions: &["dzt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
