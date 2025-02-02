use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854912: FileFormat = FileFormat {
    id: 105_854_912,
    source_type: SourceType::Wikidata,
    name: "ArcPad Layer",
    extensions: &["apl"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
