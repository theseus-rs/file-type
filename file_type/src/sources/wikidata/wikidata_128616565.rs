use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128616565: FileType = FileType {
    file_format: &FileFormat {
        id: 128_616_565,
        source_type: SourceType::Wikidata,
        name: "Asymptote file format",
        extensions: &["asy"],
        media_types: &["text/x-asymptote"],
        signatures: &[],
        related_formats: &[],
    },
};
