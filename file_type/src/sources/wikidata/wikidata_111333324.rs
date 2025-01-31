use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333324: FileFormat = FileFormat {
    id: 111_333_324,
    puid: "wikidata/111333324",
    name: "Turtle Beach Pinnacle sound bank",
    extensions: &["psb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
