use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298931: FileFormat = FileFormat {
    id: 123_298_931,
    puid: "wikidata/123298931",
    name: "Retrospect RRR File",
    extensions: &["rrr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
