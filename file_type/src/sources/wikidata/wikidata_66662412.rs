use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66662412: FileFormat = FileFormat {
    id: 66_662_412,
    source_type: SourceType::Wikidata,
    name: "ScreenCam Movies",
    extensions: &["scm"],
    media_types: &["application/vnd.lotus-screencam"],
    signatures: &[],
    related_formats: &[],
};
