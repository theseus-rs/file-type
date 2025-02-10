use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121544667: FileType = FileType {
    file_format: &FileFormat {
        id: 121_544_667,
        source_type: SourceType::Wikidata,
        name: "At Home 2009 Tax Return File",
        extensions: &["t09"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
