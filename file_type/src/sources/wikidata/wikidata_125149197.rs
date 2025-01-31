use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125149197: FileFormat = FileFormat {
    id: 125_149_197,
    puid: "wikidata/125149197",
    name: "Units Data File",
    extensions: &["units"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
