use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66439311: FileFormat = FileFormat {
    id: 66_439_311,
    puid: "wikidata/66439311",
    name: "Navy DIF",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
