use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131178576: FileFormat = FileFormat {
    id: 131_178_576,
    puid: "wikidata/131178576",
    name: "SWIG source code file",
    extensions: &["swg"],
    media_types: &["text/swig"],
    internal_signatures: &[],
    related_formats: &[],
};
