use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28756608: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_608,
        source_type: SourceType::Wikidata,
        name: "FoxPro script",
        extensions: &["prg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
