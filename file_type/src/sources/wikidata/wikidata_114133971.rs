use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114133971: FileType = FileType {
    file_format: &FileFormat {
        id: 114_133_971,
        source_type: SourceType::Wikidata,
        name: "MSI Molfile",
        extensions: &["msm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
