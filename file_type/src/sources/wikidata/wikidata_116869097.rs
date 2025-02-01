use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116869097: FileFormat = FileFormat {
    id: 116_869_097,
    puid: "wikidata/116869097",
    name: "Summitsoft Envelope",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
