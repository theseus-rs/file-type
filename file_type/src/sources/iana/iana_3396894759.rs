use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3396894759: FileFormat = FileFormat {
    id: 3_396_894_759,
    source_type: SourceType::Iana,
    name: "rpki-signed-tal",
    extensions: &[],
    media_types: &["application/rpki-signed-tal"],
    internal_signatures: &[],
    related_formats: &[],
};
