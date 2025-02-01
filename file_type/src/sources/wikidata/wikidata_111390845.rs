use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111390845: FileFormat = FileFormat {
    id: 111_390_845,
    puid: "wikidata/111390845",
    name: "Bryce Object File",
    extensions: &["obp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
