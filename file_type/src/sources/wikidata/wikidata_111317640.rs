use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317640: FileFormat = FileFormat {
    id: 111_317_640,
    puid: "wikidata/111317640",
    name: "MFi - i-Melody - Melody Format for i-Mode",
    extensions: &["mld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
