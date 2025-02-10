use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967403: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_403,
        source_type: SourceType::Wikidata,
        name: "CUD-FM-File",
        extensions: &["cff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
