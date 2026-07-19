use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_79587: FileType = FileType {
    file_format: &FileFormat {
        id: 79_587,
        source_type: SourceType::Wikidata,
        name: "paul et maelia couple goal",
        extensions: &["kkk", "kml"],
        media_types: &[
            "application/vnd.google-earth.kml+xml",
            "application/vnd.google-earth.kmz",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
