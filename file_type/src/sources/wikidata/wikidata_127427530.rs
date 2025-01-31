use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127427530: FileFormat = FileFormat {
    id: 127_427_530,
    puid: "wikidata/127427530",
    name: "GGUF",
    extensions: &["gguf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
