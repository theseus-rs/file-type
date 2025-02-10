use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117804204: FileType = FileType {
    file_format: &FileFormat {
        id: 117_804_204,
        source_type: SourceType::Wikidata,
        name: "VideoImpression File",
        extensions: &["vif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
