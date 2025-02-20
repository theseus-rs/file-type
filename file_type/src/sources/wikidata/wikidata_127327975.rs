use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127327975: FileType = FileType {
    file_format: &FileFormat {
        id: 127_327_975,
        source_type: SourceType::Wikidata,
        name: "CUDA file",
        extensions: &["cu"],
        media_types: &["text/x-cuda"],
        signatures: &[],
        related_formats: &[],
    },
};
