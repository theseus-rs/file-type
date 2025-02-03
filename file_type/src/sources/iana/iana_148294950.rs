use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_148294950: FileFormat = FileFormat {
    id: 148_294_950,
    source_type: SourceType::Iana,
    name: "vnd.mdl-mbsdf",
    extensions: &[],
    media_types: &["application/vnd.mdl-mbsdf"],
    internal_signatures: &[],
    related_formats: &[],
};
