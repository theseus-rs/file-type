use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27995530: FileType = FileType {
    file_format: &FileFormat {
        id: 27_995_530,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts BIG Archive",
        extensions: &["big", "viv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
