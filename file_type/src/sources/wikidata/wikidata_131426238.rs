use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131426238: FileType = FileType {
    file_format: &FileFormat {
        id: 131_426_238,
        source_type: SourceType::Wikidata,
        name: "Whiley file format",
        extensions: &["whiley"],
        media_types: &["text/x-whiley"],
        signatures: &[],
        related_formats: &[],
    },
};
