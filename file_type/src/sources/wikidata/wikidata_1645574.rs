use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1645574: FileType = FileType {
    file_format: &FileFormat {
        id: 1_645_574,
        source_type: SourceType::Wikidata,
        name: "Protocol Buffers",
        extensions: &["binpb", "proto", "txtpb"],
        media_types: &["application/protobuf", "application/vnd.google.protobuf"],
        signatures: &[],
        related_formats: &[],
    },
};
