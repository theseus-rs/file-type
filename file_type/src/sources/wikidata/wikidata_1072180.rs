use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1072180: FileType = FileType {
    file_format: &FileFormat {
        id: 1_072_180,
        source_type: SourceType::Wikidata,
        name: "Synchronized Multimedia Integration Language",
        extensions: &["smi", "smil"],
        media_types: &["application/smil+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
