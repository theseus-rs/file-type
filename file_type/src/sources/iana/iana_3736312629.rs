use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3736312629: FileFormat = FileFormat {
    id: 3_736_312_629,
    source_type: SourceType::Iana,
    name: "rtx",
    extensions: &[],
    media_types: &["audio/rtx"],
    signatures: &[],
    related_formats: &[],
};
