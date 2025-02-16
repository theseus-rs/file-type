use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132232221: FileType = FileType {
    file_format: &FileFormat {
        id: 132_232_221,
        source_type: SourceType::Wikidata,
        name: "Next Byte Codes file format",
        extensions: &["nbc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
