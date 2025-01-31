use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298805: FileFormat = FileFormat {
    id: 123_298_805,
    puid: "wikidata/123298805",
    name: "Retrospect RDX File",
    extensions: &["rdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
