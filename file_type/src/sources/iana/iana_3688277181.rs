use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3688277181: FileFormat = FileFormat {
    id: 3_688_277_181,
    source_type: SourceType::Iana,
    name: "flac",
    extensions: &[],
    media_types: &["audio/flac"],
    internal_signatures: &[],
    related_formats: &[],
};
