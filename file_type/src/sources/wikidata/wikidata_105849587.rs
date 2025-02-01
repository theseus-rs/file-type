use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849587: FileFormat = FileFormat {
    id: 105_849_587,
    puid: "wikidata/105849587",
    name: "Cube LUT format (with rem)",
    extensions: &["cube"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
