use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5514807: FileFormat = FileFormat {
    id: 5_514_807,
    puid: "wikidata/5514807",
    name: "GUIDO music notation",
    extensions: &["gmn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
