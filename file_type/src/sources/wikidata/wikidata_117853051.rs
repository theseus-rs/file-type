use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117853051: FileType = FileType {
    file_format: &FileFormat {
        id: 117_853_051,
        source_type: SourceType::Wikidata,
        name: "HiJaak Draw file",
        extensions: &["pdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
