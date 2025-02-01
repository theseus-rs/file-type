use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122583807: FileFormat = FileFormat {
    id: 122_583_807,
    puid: "wikidata/122583807",
    name: "Zetafax Fax Image File (Fine)",
    extensions: &["g3f"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
