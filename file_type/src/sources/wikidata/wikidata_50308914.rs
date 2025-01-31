use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50308914: FileFormat = FileFormat {
    id: 50_308_914,
    puid: "wikidata/50308914",
    name: "OMNIC Spectral Data File",
    extensions: &["spa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
