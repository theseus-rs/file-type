use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854753: FileFormat = FileFormat {
    id: 105_854_753,
    puid: "wikidata/105854753",
    name: "ArcPad Graphics layer",
    extensions: &["apg"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
