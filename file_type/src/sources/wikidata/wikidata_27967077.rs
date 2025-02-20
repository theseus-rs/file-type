use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
