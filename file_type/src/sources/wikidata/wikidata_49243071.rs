use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49243071: FileType = FileType {
    file_format: &FileFormat {
        id: 49_243_071,
        source_type: SourceType::Wikidata,
        name: "Java language source code file",
        extensions: &["java"],
        media_types: &["text/x-java"],
        signatures: &[],
        related_formats: &[],
    },
};
