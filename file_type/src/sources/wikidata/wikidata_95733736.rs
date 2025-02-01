use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95733736: FileFormat = FileFormat {
    id: 95_733_736,
    puid: "wikidata/95733736",
    name: "RealAudio Metafile",
    extensions: &["ram", "ram"],
    media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
