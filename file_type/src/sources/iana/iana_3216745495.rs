use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3216745495: FileFormat = FileFormat {
    id: 3_216_745_495,
    source_type: SourceType::Iana,
    name: "vnd.audiokoz",
    extensions: &[],
    media_types: &["audio/vnd.audiokoz"],
    internal_signatures: &[],
    related_formats: &[],
};
