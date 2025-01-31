use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854679: FileFormat = FileFormat {
    id: 105_854_679,
    puid: "wikidata/105854679",
    name: "ArcPad Stylesheet",
    extensions: &["aps"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
