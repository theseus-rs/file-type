use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206252: FileFormat = FileFormat {
    id: 28_206_252,
    puid: "wikidata/28206252",
    name: "HMR",
    extensions: &["hmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
