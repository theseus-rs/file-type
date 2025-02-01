use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777712: FileFormat = FileFormat {
    id: 28_777_712,
    puid: "wikidata/28777712",
    name: "NFF",
    extensions: &["nff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
