use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967077: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_077,
        source_type: SourceType::Wikidata,
        name: "Beathoven Synthesiser",
        extensions: &["bss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
