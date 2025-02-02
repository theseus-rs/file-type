use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66662412: FileFormat = FileFormat {
    id: 66_662_412,
    source_type: SourceType::Wikidata,
    name: "ScreenCam Movies",
    extensions: &["scm"],
    media_types: &["application/vnd.lotus-screencam"],
    internal_signatures: &[],
    related_formats: &[],
};
