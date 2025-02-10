use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17029350: FileType = FileType {
    file_format: &FileFormat {
        id: 17_029_350,
        source_type: SourceType::Wikidata,
        name: "Image Cytometry Standard",
        extensions: &["ics", "ids"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
