use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205348: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_348,
        source_type: SourceType::Wikidata,
        name: "Flexible File Format",
        extensions: &["fff"],
        media_types: &["image/x-raw-imacon"],
        signatures: &[],
        related_formats: &[],
    },
};
