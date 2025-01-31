use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2115: FileFormat = FileFormat {
    id: 2_115,
    puid: "wikidata/2115",
    name: "XML",
    extensions: &["xml", "xml"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
