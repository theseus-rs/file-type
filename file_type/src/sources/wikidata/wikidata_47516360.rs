use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47516360: FileFormat = FileFormat {
    id: 47_516_360,
    puid: "wikidata/47516360",
    name: "Statistical Analysis System Catalog XPT (Unix) v.9.1",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
