use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2347127: FileFormat = FileFormat {
    id: 2_347_127,
    puid: "wikidata/2347127",
    name: "Compressed image format",
    extensions: &["cso"],
    media_types: &["application/x-compressed-iso"],
    internal_signatures: &[],
    related_formats: &[],
};
