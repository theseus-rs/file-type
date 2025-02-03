use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2922182229: FileFormat = FileFormat {
    id: 2_922_182_229,
    source_type: SourceType::Iana,
    name: "vnd.dlna.mpeg-tts",
    extensions: &[],
    media_types: &["video/vnd.dlna.mpeg-tts"],
    internal_signatures: &[],
    related_formats: &[],
};
