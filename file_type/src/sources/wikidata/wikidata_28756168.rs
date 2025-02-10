use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28756168: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_168,
        source_type: SourceType::Wikidata,
        name: "FWKCS SRT file",
        extensions: &["srt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
