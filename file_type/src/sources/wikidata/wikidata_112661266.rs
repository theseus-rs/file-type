use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112661266: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_266,
        source_type: SourceType::Wikidata,
        name: "Lightscape Preparation File",
        extensions: &["lp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
