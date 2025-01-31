use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100323885: FileFormat = FileFormat {
    id: 100_323_885,
    puid: "wikidata/100323885",
    name: "Corel Gallery Clipart",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
