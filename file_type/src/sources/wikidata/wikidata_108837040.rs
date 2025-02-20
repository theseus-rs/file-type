use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837040: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_040,
        source_type: SourceType::Wikidata,
        name: "Nero DVD-Video Compilation File",
        extensions: &["nrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
