use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117352081: FileFormat = FileFormat {
    id: 117_352_081,
    puid: "wikidata/117352081",
    name: "Capture Library",
    extensions: &["olb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
