use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126165090: FileType = FileType {
    file_format: &FileFormat {
        id: 126_165_090,
        source_type: SourceType::Wikidata,
        name: "Husqvarna-Viking Designer 1 Stitch File",
        extensions: &["mhv", "phv", "shv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
