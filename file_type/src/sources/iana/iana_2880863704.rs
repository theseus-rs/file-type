use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2880863704: FileFormat = FileFormat {
    id: 2_880_863_704,
    source_type: SourceType::Iana,
    name: "tlsrpt+gzip",
    extensions: &[],
    media_types: &["application/tlsrpt+gzip"],
    signatures: &[],
    related_formats: &[],
};
