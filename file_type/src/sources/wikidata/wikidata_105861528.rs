use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861528: FileFormat = FileFormat {
    id: 105_861_528,
    puid: "wikidata/105861528",
    name: "Lighthouse Project",
    extensions: &["lighthouse-project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
