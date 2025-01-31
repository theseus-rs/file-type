use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206493: FileFormat = FileFormat {
    id: 28_206_493,
    puid: "wikidata/28206493",
    name: "Lightning Strike",
    extensions: &["cod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
