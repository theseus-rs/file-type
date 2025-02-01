use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123436243: FileFormat = FileFormat {
    id: 123_436_243,
    puid: "wikidata/123436243",
    name: "CD Style Sheet file",
    extensions: &["cds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
