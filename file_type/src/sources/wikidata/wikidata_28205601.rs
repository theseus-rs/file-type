use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205601: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_601,
        source_type: SourceType::Wikidata,
        name: "RIPscrip version 1 Icon",
        extensions: &["icn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
