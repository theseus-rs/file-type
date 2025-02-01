use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21620033: FileFormat = FileFormat {
    id: 21_620_033,
    puid: "wikidata/21620033",
    name: "XDMF",
    extensions: &["xdmf", "xmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
