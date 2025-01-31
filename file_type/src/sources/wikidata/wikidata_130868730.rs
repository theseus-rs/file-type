use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130868730: FileFormat = FileFormat {
    id: 130_868_730,
    puid: "wikidata/130868730",
    name: "ShExC file",
    extensions: &["shex"],
    media_types: &["text/shex"],
    internal_signatures: &[],
    related_formats: &[],
};
