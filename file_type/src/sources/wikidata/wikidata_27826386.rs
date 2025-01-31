use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826386: FileFormat = FileFormat {
    id: 27_826_386,
    puid: "wikidata/27826386",
    name: "OVR pyramid file",
    extensions: &["ovr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
