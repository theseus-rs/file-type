use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206936: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_936,
        source_type: SourceType::Wikidata,
        name: "Photo CD",
        extensions: &["pcd"],
        media_types: &["image/x-photo-cd"],
        signatures: &[],
        related_formats: &[],
    },
};
