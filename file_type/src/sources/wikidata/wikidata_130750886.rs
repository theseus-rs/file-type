use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130750886: FileFormat = FileFormat {
    id: 130_750_886,
    puid: "wikidata/130750886",
    name: "Sed script file",
    extensions: &["sed"],
    media_types: &["text/x-sed"],
    internal_signatures: &[],
    related_formats: &[],
};
