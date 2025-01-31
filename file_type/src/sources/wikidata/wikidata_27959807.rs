use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959807: FileFormat = FileFormat {
    id: 27_959_807,
    puid: "wikidata/27959807",
    name: "Ableton Live Pack",
    extensions: &["alp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
