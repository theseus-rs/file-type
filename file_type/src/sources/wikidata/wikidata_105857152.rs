use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857152: FileFormat = FileFormat {
    id: 105_857_152,
    puid: "wikidata/105857152",
    name: "MAME Hash",
    extensions: &["hsi"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
