use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83442984: FileFormat = FileFormat {
    id: 83_442_984,
    puid: "wikidata/83442984",
    name: "Ducati Data Analyzer",
    extensions: &["dda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
