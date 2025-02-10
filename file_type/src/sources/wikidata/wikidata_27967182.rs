use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967182: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_182,
        source_type: SourceType::Wikidata,
        name: "Farandole Composer sample",
        extensions: &["fsm", "usm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
