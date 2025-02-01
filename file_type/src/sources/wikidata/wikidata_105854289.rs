use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854289: FileFormat = FileFormat {
    id: 105_854_289,
    puid: "wikidata/105854289",
    name: "ArcPad configuration",
    extensions: &["apx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
