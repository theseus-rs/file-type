use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114795676: FileFormat = FileFormat {
    id: 114_795_676,
    puid: "wikidata/114795676",
    name: "ReadCube Enhanced PDF",
    extensions: &["epdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
