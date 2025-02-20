use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_836370: FileType = FileType {
    file_format: &FileFormat {
        id: 836_370,
        source_type: SourceType::Wikidata,
        name: "OPML",
        extensions: &["opml"],
        media_types: &[
            "application/xml",
            "text/x-opml",
            "text/x-opml+xml",
            "text/xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
