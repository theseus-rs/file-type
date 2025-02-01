use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206498: FileFormat = FileFormat {
    id: 28_206_498,
    puid: "wikidata/28206498",
    name: "Age of Empires Graphics File",
    extensions: &["slp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
