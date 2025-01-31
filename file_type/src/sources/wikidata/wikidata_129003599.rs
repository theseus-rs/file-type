use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129003599: FileFormat = FileFormat {
    id: 129_003_599,
    puid: "wikidata/129003599",
    name: "eC source code file",
    extensions: &["ec", "ec"],
    media_types: &["text/x-echdr", "text/x-ecsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
