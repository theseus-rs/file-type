use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118140187: FileFormat = FileFormat {
    id: 118_140_187,
    puid: "wikidata/118140187",
    name: "Serenade Symbol File",
    extensions: &["sym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
