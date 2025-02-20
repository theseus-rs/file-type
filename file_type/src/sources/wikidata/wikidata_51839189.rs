use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51839189: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_189,
        source_type: SourceType::Wikidata,
        name: "Broderbund Print Shop Deluxe Pamphlet",
        extensions: &["pdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
