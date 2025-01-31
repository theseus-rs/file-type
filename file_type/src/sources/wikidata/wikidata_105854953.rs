use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854953: FileFormat = FileFormat {
    id: 105_854_953,
    puid: "wikidata/105854953",
    name: "ArcPad preferences",
    extensions: &["apx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
