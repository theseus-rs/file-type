use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966984: FileFormat = FileFormat {
    id: 27_966_984,
    puid: "wikidata/27966984",
    name: "Actionamics Sound Tool",
    extensions: &["ast"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
