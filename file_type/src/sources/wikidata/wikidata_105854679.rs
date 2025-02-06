use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854679: FileFormat = FileFormat {
    id: 105_854_679,
    source_type: SourceType::Wikidata,
    name: "ArcPad Stylesheet",
    extensions: &["aps"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
