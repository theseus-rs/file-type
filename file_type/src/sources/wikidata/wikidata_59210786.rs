use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59210786: FileFormat = FileFormat {
    id: 59_210_786,
    puid: "wikidata/59210786",
    name: "BIM Collaboration Format",
    extensions: &["bcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
