use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169695: FileFormat = FileFormat {
    id: 122_169_695,
    puid: "wikidata/122169695",
    name: "Key Cache File",
    extensions: &["ekc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
