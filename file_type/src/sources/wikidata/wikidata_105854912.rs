use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854912: FileFormat = FileFormat {
    id: 105_854_912,
    puid: "wikidata/105854912",
    name: "ArcPad Layer",
    extensions: &["apl"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
