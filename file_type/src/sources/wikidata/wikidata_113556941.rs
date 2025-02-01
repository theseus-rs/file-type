use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113556941: FileFormat = FileFormat {
    id: 113_556_941,
    puid: "wikidata/113556941",
    name: "CDR-Win Image",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
