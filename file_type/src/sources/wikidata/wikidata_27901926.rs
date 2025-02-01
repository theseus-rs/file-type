use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901926: FileFormat = FileFormat {
    id: 27_901_926,
    puid: "wikidata/27901926",
    name: "Amateur Data Interchange Format, version 2.2.6",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
