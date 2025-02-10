use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34290760: FileType = FileType {
    file_format: &FileFormat {
        id: 34_290_760,
        source_type: SourceType::Wikidata,
        name: "Statistical Package for the Social Sciences syntax file",
        extensions: &["sps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
