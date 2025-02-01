use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130241065: FileFormat = FileFormat {
    id: 130_241_065,
    puid: "wikidata/130241065",
    name: "Literate Idris source code file",
    extensions: &["lidr"],
    media_types: &["text/x-literate-idris"],
    internal_signatures: &[],
    related_formats: &[],
};
