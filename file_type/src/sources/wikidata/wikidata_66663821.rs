use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66663821: FileFormat = FileFormat {
    id: 66_663_821,
    puid: "wikidata/66663821",
    name: "Hewlett-Packard Graphics Language format",
    extensions: &["hgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
