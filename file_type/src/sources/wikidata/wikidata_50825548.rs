use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50825548: FileType = FileType {
    file_format: &FileFormat {
        id: 50_825_548,
        source_type: SourceType::Wikidata,
        name: "AVCHD Playlist File",
        extensions: &["mpl", "mpls"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
