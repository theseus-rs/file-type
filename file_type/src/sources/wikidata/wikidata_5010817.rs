use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5010817: FileType = FileType {
    file_format: &FileFormat {
        id: 5_010_817,
        source_type: SourceType::Wikidata,
        name: "CFS",
        extensions: &["cfs"],
        media_types: &["application/x-cfs-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
