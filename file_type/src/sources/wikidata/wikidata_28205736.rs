use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205736: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_736,
        source_type: SourceType::Wikidata,
        name: "Award BIOS logo, version 2",
        extensions: &["bmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
