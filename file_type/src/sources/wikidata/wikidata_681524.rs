use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_681524: FileType = FileType {
    file_format: &FileFormat {
        id: 681_524,
        source_type: SourceType::Wikidata,
        name: "XML Shareable Playlist Format",
        extensions: &["xspf"],
        media_types: &["application/xspf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
