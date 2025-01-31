use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130642271: FileFormat = FileFormat {
    id: 130_642_271,
    puid: "wikidata/130642271",
    name: "Roboconf graph file",
    extensions: &["graph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
