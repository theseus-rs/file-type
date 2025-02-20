use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_7: FileType = FileType {
    file_format: &FileFormat {
        id: 7,
        source_type: SourceType::Linguist,
        name: "ASN.1",
        extensions: &["asn", "asn1"],
        media_types: &["text/x-ttcn-asn"],
        signatures: &[],
        related_formats: &[],
    },
};
