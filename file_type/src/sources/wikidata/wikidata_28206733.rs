use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206733: FileFormat = FileFormat {
    id: 28_206_733,
    puid: "wikidata/28206733",
    name: "Newsroom",
    extensions: &["nsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
