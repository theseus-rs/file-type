use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95733178: FileFormat = FileFormat {
    id: 95_733_178,
    puid: "wikidata/95733178",
    name: "RealAudio version 4",
    extensions: &["ra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
