use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417253: FileFormat = FileFormat {
    id: 111_417_253,
    puid: "wikidata/111417253",
    name: "Resource Script format",
    extensions: &["rc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
