use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2676856122: FileFormat = FileFormat {
    id: 2_676_856_122,
    source_type: SourceType::Iana,
    name: "vnd.directv.mpeg-tts",
    extensions: &[],
    media_types: &["video/vnd.directv.mpeg-tts"],
    signatures: &[],
    related_formats: &[],
};
