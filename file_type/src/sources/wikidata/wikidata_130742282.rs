use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130742282: FileType = FileType {
    file_format: &FileFormat {
        id: 130_742_282,
        source_type: SourceType::Wikidata,
        name: "scdoc file format",
        extensions: &["scd", "scdoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
