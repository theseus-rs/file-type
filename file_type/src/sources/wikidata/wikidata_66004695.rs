use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66004695: FileType = FileType {
    file_format: &FileFormat {
        id: 66_004_695,
        source_type: SourceType::Wikidata,
        name: "ImageStyler File",
        extensions: &["ist"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
