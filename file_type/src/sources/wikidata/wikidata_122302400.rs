use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122302400: FileFormat = FileFormat {
    id: 122_302_400,
    puid: "wikidata/122302400",
    name: "HLD File",
    extensions: &["hld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
