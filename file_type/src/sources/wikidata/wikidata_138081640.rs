use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138081640: FileType = FileType {
    file_format: &FileFormat {
        id: 138_081_640,
        source_type: SourceType::Wikidata,
        name: "XFL",
        extensions: &["xfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
