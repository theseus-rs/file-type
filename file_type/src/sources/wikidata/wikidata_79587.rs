use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_79587: FileType = FileType {
    file_format: &FileFormat {
        id: 79_587,
        source_type: SourceType::Wikidata,
        name: "Keyhole Markup Language",
        extensions: &["kml", "kmz"],
        media_types: &[
            "application/vnd.google-earth.kml+xml",
            "application/vnd.google-earth.kmz",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
