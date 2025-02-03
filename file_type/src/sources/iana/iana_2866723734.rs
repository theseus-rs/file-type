use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2866723734: FileFormat = FileFormat {
    id: 2_866_723_734,
    source_type: SourceType::Iana,
    name: "byteranges",
    extensions: &[],
    media_types: &["multipart/byteranges"],
    internal_signatures: &[],
    related_formats: &[],
};
