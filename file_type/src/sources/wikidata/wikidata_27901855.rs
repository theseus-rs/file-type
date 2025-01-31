use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901855: FileFormat = FileFormat {
    id: 27_901_855,
    puid: "wikidata/27901855",
    name: "Amateur Data Interchange Format, version 2.1.8",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
