use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72273742: FileType = FileType {
    file_format: &FileFormat {
        id: 72_273_742,
        source_type: SourceType::Wikidata,
        name: "TPN",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
