use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27441566: FileType = FileType {
    file_format: &FileFormat {
        id: 27_441_566,
        source_type: SourceType::Wikidata,
        name: "LAS file format",
        extensions: &["las", "laz"],
        media_types: &["application/vnd.las"],
        signatures: &[],
        related_formats: &[],
    },
};
