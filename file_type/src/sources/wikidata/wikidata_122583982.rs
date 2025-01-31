use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122583982: FileFormat = FileFormat {
    id: 122_583_982,
    puid: "wikidata/122583982",
    name: "Zetafax Fax Image File (Normal)",
    extensions: &["g3n"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
