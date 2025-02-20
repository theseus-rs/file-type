use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_376852: FileType = FileType {
    file_format: &FileFormat {
        id: 376_852,
        source_type: SourceType::Wikidata,
        name: "Extended Module",
        extensions: &["xm"],
        media_types: &["audio/xm"],
        signatures: &[],
        related_formats: &[],
    },
};
