use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5299371: FileFormat = FileFormat {
    id: 5_299_371,
    puid: "wikidata/5299371",
    name: "dotXSI",
    extensions: &["xsi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
