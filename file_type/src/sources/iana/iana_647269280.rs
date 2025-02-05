use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_647269280: FileFormat = FileFormat {
    id: 647_269_280,
    source_type: SourceType::Iana,
    name: "mpeg4-iod-xmt",
    extensions: &[],
    media_types: &["application/mpeg4-iod-xmt"],
    signatures: &[],
    related_formats: &[],
};
