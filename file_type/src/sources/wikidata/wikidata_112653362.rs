use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112653362: FileFormat = FileFormat {
    id: 112_653_362,
    puid: "wikidata/112653362",
    name: "Astound Draw file",
    extensions: &["adw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
