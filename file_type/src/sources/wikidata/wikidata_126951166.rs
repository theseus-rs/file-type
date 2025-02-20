use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951166: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_166,
        source_type: SourceType::Wikidata,
        name: "Groovy script file",
        extensions: &["groovy", "gsh", "gvy", "gy"],
        media_types: &["text/x-groovy"],
        signatures: &[],
        related_formats: &[],
    },
};
