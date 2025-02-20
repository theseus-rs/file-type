use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50258969: FileType = FileType {
    file_format: &FileFormat {
        id: 50_258_969,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Stencil, version 2013",
        extensions: &["vssx"],
        media_types: &["application/vnd.ms-visio.stencil.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
