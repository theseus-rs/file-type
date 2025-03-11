use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7391977: FileType = FileType {
    file_format: &FileFormat {
        id: 7_391_977,
        source_type: SourceType::Wikidata,
        name: "SOAP Service Description Language",
        extensions: &[],
        media_types: &["application/ssdl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
