use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112660704: FileType = FileType {
    file_format: &FileFormat {
        id: 112_660_704,
        source_type: SourceType::Wikidata,
        name: "Portfolio File",
        extensions: &["bfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
