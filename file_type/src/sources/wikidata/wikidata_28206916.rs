use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206916: FileFormat = FileFormat {
    id: 28_206_916,
    puid: "wikidata/28206916",
    name: "Portfolio Graphics",
    extensions: &["pgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
