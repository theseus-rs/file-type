use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919086: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_086,
        source_type: SourceType::Wikidata,
        name: "CMX 3600 edit decision list",
        extensions: &["edl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
