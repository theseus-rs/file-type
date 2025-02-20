use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049770: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_770,
        source_type: SourceType::Wikidata,
        name: "DKBTrace scene description",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
