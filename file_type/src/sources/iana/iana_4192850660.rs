use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4192850660: FileFormat = FileFormat {
    id: 4_192_850_660,
    source_type: SourceType::Iana,
    name: "rtp-midi",
    extensions: &[],
    media_types: &["audio/rtp-midi"],
    internal_signatures: &[],
    related_formats: &[],
};
