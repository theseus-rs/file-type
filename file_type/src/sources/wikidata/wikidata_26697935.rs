use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26697935: FileFormat = FileFormat {
    id: 26_697_935,
    puid: "wikidata/26697935",
    name: "PHP script",
    extensions: &["php"],
    media_types: &["text/x-php"],
    internal_signatures: &[],
    related_formats: &[],
};
