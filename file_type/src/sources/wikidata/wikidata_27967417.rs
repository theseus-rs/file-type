use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967417: FileFormat = FileFormat {
    id: 27_967_417,
    puid: "wikidata/27967417",
    name: "Callus OPL Register Log",
    extensions: &["cym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
