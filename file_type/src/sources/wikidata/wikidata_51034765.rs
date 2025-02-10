use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51034765: FileType = FileType {
    file_format: &FileFormat {
        id: 51_034_765,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 10",
        extensions: &["pspimage"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
