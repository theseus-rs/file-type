use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901929: FileFormat = FileFormat {
    id: 27_901_929,
    puid: "wikidata/27901929",
    name: "Amateur Data Interchange Format, version 2.2.7",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
