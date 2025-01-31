use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757983: FileFormat = FileFormat {
    id: 28_757_983,
    puid: "wikidata/28757983",
    name: "IPF",
    extensions: &["ipf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
