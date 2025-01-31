use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951310: FileFormat = FileFormat {
    id: 126_951_310,
    puid: "wikidata/126951310",
    name: "Haskell Script File Format",
    extensions: &["hs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
