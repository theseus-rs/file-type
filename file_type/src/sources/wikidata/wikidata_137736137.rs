use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137736137: FileType = FileType {
    file_format: &FileFormat {
        id: 137_736_137,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate style file",
        extensions: &["cps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
