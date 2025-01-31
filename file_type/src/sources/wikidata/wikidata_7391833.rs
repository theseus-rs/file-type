use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7391833: FileFormat = FileFormat {
    id: 7_391_833,
    puid: "wikidata/7391833",
    name: "SND",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
