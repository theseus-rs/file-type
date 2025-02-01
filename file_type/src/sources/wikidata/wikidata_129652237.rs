use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129652237: FileFormat = FileFormat {
    id: 129_652_237,
    puid: "wikidata/129652237",
    name: "Igor Pro procedure file",
    extensions: &["ipf"],
    media_types: &["text/ipf"],
    internal_signatures: &[],
    related_formats: &[],
};
