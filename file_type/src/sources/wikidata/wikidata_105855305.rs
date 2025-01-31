use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855305: FileFormat = FileFormat {
    id: 105_855_305,
    puid: "wikidata/105855305",
    name: "Fritzing sketch",
    extensions: &["fz"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
