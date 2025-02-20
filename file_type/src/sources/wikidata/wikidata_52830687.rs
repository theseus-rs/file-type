use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52830687: FileType = FileType {
    file_format: &FileFormat {
        id: 52_830_687,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 8",
        extensions: &["pspimage"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
