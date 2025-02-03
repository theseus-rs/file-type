use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2341024932: FileFormat = FileFormat {
    id: 2_341_024_932,
    source_type: SourceType::Iana,
    name: "vnd.senx.warpscript",
    extensions: &[],
    media_types: &["text/vnd.senx.warpscript"],
    internal_signatures: &[],
    related_formats: &[],
};
