use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996222: FileFormat = FileFormat {
    id: 27_996_222,
    puid: "wikidata/27996222",
    name: "Fallout character description",
    extensions: &["gcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
