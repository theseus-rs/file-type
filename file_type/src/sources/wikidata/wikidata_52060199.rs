use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52060199: FileType = FileType {
    file_format: &FileFormat {
        id: 52_060_199,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 7",
        extensions: &["psp", "pspimage"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
