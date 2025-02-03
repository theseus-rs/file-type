use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1447934773: FileFormat = FileFormat {
    id: 1_447_934_773,
    source_type: SourceType::Iana,
    name: "vnd.iptvforum.ttsavc",
    extensions: &[],
    media_types: &["video/vnd.iptvforum.ttsavc"],
    internal_signatures: &[],
    related_formats: &[],
};
