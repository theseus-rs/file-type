use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2379975098: FileType = FileType {
    file_format: &FileFormat {
        id: 2_379_975_098,
        source_type: SourceType::Iana,
        name: "vnd.ipld.dag-cbor",
        extensions: &[],
        media_types: &["application/vnd.ipld.dag-cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
