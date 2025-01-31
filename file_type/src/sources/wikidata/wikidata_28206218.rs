use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206218: FileFormat = FileFormat {
    id: 28_206_218,
    puid: "wikidata/28206218",
    name: "GRF",
    extensions: &["grf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
