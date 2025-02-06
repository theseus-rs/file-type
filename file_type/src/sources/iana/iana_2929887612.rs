use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2929887612: FileFormat = FileFormat {
    id: 2_929_887_612,
    source_type: SourceType::Iana,
    name: "urc-uisocketdesc+xml",
    extensions: &[],
    media_types: &["application/urc-uisocketdesc+xml"],
    signatures: &[],
    related_formats: &[],
};
