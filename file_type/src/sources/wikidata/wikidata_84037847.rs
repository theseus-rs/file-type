use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84037847: FileType = FileType {
    file_format: &FileFormat {
        id: 84_037_847,
        source_type: SourceType::Wikidata,
        name: "WARC 1.1",
        extensions: &["warc"],
        media_types: &["application/warc"],
        signatures: &[],
        related_formats: &[],
    },
};
