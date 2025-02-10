use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967541: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_541,
        source_type: SourceType::Wikidata,
        name: "IFF-DEEP",
        extensions: &["deep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
