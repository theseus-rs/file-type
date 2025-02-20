use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51034568: FileType = FileType {
    file_format: &FileFormat {
        id: 51_034_568,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 9",
        extensions: &["pspimage"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
