use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111190501: FileFormat = FileFormat {
    id: 111_190_501,
    puid: "wikidata/111190501",
    name: "Visual Tool Markup Language Document",
    extensions: &["vtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
