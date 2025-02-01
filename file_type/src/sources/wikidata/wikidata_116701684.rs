use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116701684: FileFormat = FileFormat {
    id: 116_701_684,
    puid: "wikidata/116701684",
    name: "Mascot Generic Format",
    extensions: &["mgf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
