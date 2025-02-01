use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50183465: FileFormat = FileFormat {
    id: 50_183_465,
    puid: "wikidata/50183465",
    name: "AXD HTTP Handler File",
    extensions: &["axd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
