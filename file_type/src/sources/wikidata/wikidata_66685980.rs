use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66685980: FileFormat = FileFormat {
    id: 66_685_980,
    puid: "wikidata/66685980",
    name: "OR2",
    extensions: &["or2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
