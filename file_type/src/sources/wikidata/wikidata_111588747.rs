use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111588747: FileFormat = FileFormat {
    id: 111_588_747,
    puid: "wikidata/111588747",
    name: "Inspiration Software File",
    extensions: &["isf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
