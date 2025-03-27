use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50339410: FileType = FileType {
    file_format: &FileFormat {
        id: 50_339_410,
        source_type: SourceType::Wikidata,
        name: "Expert Witness Disk Image",
        extensions: &["e01"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
