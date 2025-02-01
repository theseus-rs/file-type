use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128775907: FileFormat = FileFormat {
    id: 128_775_907,
    puid: "wikidata/128775907",
    name: "Coq file format",
    extensions: &["v"],
    media_types: &["text/x-coq"],
    internal_signatures: &[],
    related_formats: &[],
};
