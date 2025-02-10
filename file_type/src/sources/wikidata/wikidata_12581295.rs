use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_12581295: FileType = FileType {
    file_format: &FileFormat {
        id: 12_581_295,
        source_type: SourceType::Wikidata,
        name: "KT 3G video file format",
        extensions: &["k3g"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
