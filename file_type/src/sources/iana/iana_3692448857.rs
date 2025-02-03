use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3692448857: FileFormat = FileFormat {
    id: 3_692_448_857,
    source_type: SourceType::Iana,
    name: "vnd.pg.osasli",
    extensions: &[],
    media_types: &["application/vnd.pg.osasli"],
    internal_signatures: &[],
    related_formats: &[],
};
