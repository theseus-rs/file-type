use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298834: FileFormat = FileFormat {
    id: 123_298_834,
    puid: "wikidata/123298834",
    name: "Retrospect UTX File",
    extensions: &["utx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
