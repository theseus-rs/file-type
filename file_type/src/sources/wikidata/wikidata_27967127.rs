use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967127: FileFormat = FileFormat {
    id: 27_967_127,
    puid: "wikidata/27967127",
    name: "CMS",
    extensions: &["cms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
