use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29896310: FileFormat = FileFormat {
    id: 29_896_310,
    puid: "wikidata/29896310",
    name: "ABIF",
    extensions: &["ab1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
