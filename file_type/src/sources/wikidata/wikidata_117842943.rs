use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117842943: FileFormat = FileFormat {
    id: 117_842_943,
    puid: "wikidata/117842943",
    name: "Everex Everfax 24/96",
    extensions: &["ef3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
