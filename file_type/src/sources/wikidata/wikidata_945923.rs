use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_945923: FileFormat = FileFormat {
    id: 945_923,
    puid: "wikidata/945923",
    name: "Web application ARchive",
    extensions: &["war"],
    media_types: &["application/java-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
