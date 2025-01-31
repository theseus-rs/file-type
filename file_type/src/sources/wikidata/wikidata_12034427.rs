use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_12034427: FileFormat = FileFormat {
    id: 12_034_427,
    puid: "wikidata/12034427",
    name: "LuraDocument Format",
    extensions: &["ldf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
