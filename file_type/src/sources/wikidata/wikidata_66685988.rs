use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66685988: FileFormat = FileFormat {
    id: 66_685_988,
    puid: "wikidata/66685988",
    name: "OR5",
    extensions: &["or5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
