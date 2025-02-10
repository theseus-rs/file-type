use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120750742: FileType = FileType {
    file_format: &FileFormat {
        id: 120_750_742,
        source_type: SourceType::Wikidata,
        name: "OpenRP",
        extensions: &["rp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
