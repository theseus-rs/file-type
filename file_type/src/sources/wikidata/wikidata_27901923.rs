use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901923: FileFormat = FileFormat {
    id: 27_901_923,
    puid: "wikidata/27901923",
    name: "Amateur Data Interchange Format, version 2.2.5",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
