use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3395190901: FileFormat = FileFormat {
    id: 3_395_190_901,
    source_type: SourceType::Iana,
    name: "ISUP",
    extensions: &[],
    media_types: &["application/ISUP"],
    signatures: &[],
    related_formats: &[],
};
