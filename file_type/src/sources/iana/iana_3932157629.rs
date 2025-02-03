use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3932157629: FileFormat = FileFormat {
    id: 3_932_157_629,
    source_type: SourceType::Iana,
    name: "font-tdpfr",
    extensions: &[],
    media_types: &["application/font-tdpfr"],
    internal_signatures: &[],
    related_formats: &[],
};
