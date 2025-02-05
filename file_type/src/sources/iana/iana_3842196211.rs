use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3842196211: FileFormat = FileFormat {
    id: 3_842_196_211,
    source_type: SourceType::Iana,
    name: "vnd.ms-wmdrm.lic-resp",
    extensions: &[],
    media_types: &["application/vnd.ms-wmdrm.lic-resp"],
    signatures: &[],
    related_formats: &[],
};
