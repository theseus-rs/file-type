use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333309: FileFormat = FileFormat {
    id: 111_333_309,
    puid: "wikidata/111333309",
    name: "Turtle Beach Pinnacle program file",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
