use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129177252: FileFormat = FileFormat {
    id: 129_177_252,
    puid: "wikidata/129177252",
    name: "Felix source code file",
    extensions: &["flx"],
    media_types: &["text/x-felix"],
    internal_signatures: &[],
    related_formats: &[],
};
