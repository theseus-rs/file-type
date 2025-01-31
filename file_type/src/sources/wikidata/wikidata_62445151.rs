use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62445151: FileFormat = FileFormat {
    id: 62_445_151,
    puid: "wikidata/62445151",
    name: "OWL Functional-Style Syntax",
    extensions: &["ofn"],
    media_types: &["text/owl-functional"],
    internal_signatures: &[],
    related_formats: &[],
};
