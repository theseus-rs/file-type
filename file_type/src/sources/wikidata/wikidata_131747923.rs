use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131747923: FileType = FileType {
    file_format: &FileFormat {
        id: 131_747_923,
        source_type: SourceType::Wikidata,
        name: "MNI polygonal surface mesh format",
        extensions: &["obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
