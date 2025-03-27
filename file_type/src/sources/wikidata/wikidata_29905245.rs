use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905245: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_245,
        source_type: SourceType::Wikidata,
        name: "StarOffice Draw, version 5.x",
        extensions: &["sda"],
        media_types: &[
            "application/vnd.stardivision.draw",
            "application/x-stardraw",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
