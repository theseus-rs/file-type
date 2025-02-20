use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111262619: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_619,
        source_type: SourceType::Wikidata,
        name: "Raw Yamaha DX7 32-voice data",
        extensions: &["32", "dx7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
