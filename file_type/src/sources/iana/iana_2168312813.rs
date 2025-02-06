use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2168312813: FileFormat = FileFormat {
    id: 2_168_312_813,
    source_type: SourceType::Iana,
    name: "SMPTE292M",
    extensions: &[],
    media_types: &["video/SMPTE292M"],
    signatures: &[],
    related_formats: &[],
};
