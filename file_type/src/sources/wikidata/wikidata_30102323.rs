use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_30102323: FileFormat = FileFormat {
    id: 30_102_323,
    puid: "wikidata/30102323",
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.5",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
