use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600228: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_228,
        source_type: SourceType::Wikidata,
        name: "APL workspace",
        extensions: &["apl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
