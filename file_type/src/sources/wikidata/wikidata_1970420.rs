use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1970420: FileType = FileType {
    file_format: &FileFormat {
        id: 1_970_420,
        source_type: SourceType::Wikidata,
        name: "Simple file verification",
        extensions: &["sfv"],
        media_types: &["text/x-sfv"],
        signatures: &[],
        related_formats: &[],
    },
};
