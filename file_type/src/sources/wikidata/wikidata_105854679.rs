use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854679: FileFormat = FileFormat {
    id: 105_854_679,
    source_type: SourceType::Wikidata,
    name: "ArcPad Stylesheet",
    extensions: &["aps"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
