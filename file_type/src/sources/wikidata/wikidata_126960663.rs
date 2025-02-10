use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126960663: FileType = FileType {
    file_format: &FileFormat {
        id: 126_960_663,
        source_type: SourceType::Wikidata,
        name: "Apache Thrift file",
        extensions: &["thrift"],
        media_types: &["application/x-thrift"],
        signatures: &[],
        related_formats: &[],
    },
};
