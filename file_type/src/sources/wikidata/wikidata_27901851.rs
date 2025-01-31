use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901851: FileFormat = FileFormat {
    id: 27_901_851,
    puid: "wikidata/27901851",
    name: "Amateur Data Interchange Format, version 2.1.6",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
