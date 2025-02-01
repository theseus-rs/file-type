use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901919: FileFormat = FileFormat {
    id: 27_901_919,
    puid: "wikidata/27901919",
    name: "Amateur Data Interchange Format, version 2.2.1",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
