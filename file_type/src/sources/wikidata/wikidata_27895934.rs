use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27895934: FileFormat = FileFormat {
    id: 27_895_934,
    puid: "wikidata/27895934",
    name: "Amateur Data Interchange Format, version 1",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
