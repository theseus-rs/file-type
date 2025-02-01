use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66662412: FileFormat = FileFormat {
    id: 66_662_412,
    puid: "wikidata/66662412",
    name: "ScreenCam Movies",
    extensions: &["scm"],
    media_types: &["application/vnd.lotus-screencam"],
    internal_signatures: &[],
    related_formats: &[],
};
