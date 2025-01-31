use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122829936: FileFormat = FileFormat {
    id: 122_829_936,
    puid: "wikidata/122829936",
    name: "Creativity Workshop PWK file",
    extensions: &["pwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
