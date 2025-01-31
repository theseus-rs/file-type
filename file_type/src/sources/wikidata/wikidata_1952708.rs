use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1952708: FileFormat = FileFormat {
    id: 1_952_708,
    puid: "wikidata/1952708",
    name: "FILE_ID.DIZ",
    extensions: &["diz"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
