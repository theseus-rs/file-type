use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3630199126: FileFormat = FileFormat {
    id: 3_630_199_126,
    source_type: SourceType::Iana,
    name: "vnd.vcx",
    extensions: &[],
    media_types: &["application/vnd.vcx"],
    internal_signatures: &[],
    related_formats: &[],
};
