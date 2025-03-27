use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3341814: FileType = FileType {
    file_format: &FileFormat {
        id: 3_341_814,
        source_type: SourceType::Wikidata,
        name: "Nikon Electronic File",
        extensions: &["nef"],
        media_types: &["image/x-nikon-nef", "image/x-raw-nikon"],
        signatures: &[],
        related_formats: &[],
    },
};
