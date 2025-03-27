use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_296195: FileType = FileType {
    file_format: &FileFormat {
        id: 296_195,
        source_type: SourceType::Wikidata,
        name: "Attention Profiling Mark-up Language",
        extensions: &[],
        media_types: &["application/xml+apml"],
        signatures: &[],
        related_formats: &[],
    },
};
