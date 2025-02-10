use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121837535: FileType = FileType {
    file_format: &FileFormat {
        id: 121_837_535,
        source_type: SourceType::Wikidata,
        name: "OPML File 1.x",
        extensions: &["opml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
