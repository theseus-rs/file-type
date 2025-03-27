use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905223: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_223,
        source_type: SourceType::Wikidata,
        name: "StarOffice Draw, version 3.x",
        extensions: &["sda"],
        media_types: &[
            "application/vnd.stardivision.draw",
            "application/x-stardraw",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
