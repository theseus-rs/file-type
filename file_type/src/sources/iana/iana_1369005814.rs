use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1369005814: FileFormat = FileFormat {
    id: 1_369_005_814,
    source_type: SourceType::Iana,
    name: "SMV0",
    extensions: &[],
    media_types: &["audio/SMV0"],
    signatures: &[],
    related_formats: &[],
};
