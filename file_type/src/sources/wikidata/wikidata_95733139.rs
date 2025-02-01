use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95733139: FileFormat = FileFormat {
    id: 95_733_139,
    puid: "wikidata/95733139",
    name: "RealAudio version 3",
    extensions: &["ra"],
    media_types: &["audio/vnd.rn-realaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
