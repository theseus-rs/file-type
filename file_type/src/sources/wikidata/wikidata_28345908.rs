use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28345908: FileFormat = FileFormat {
    id: 28_345_908,
    puid: "wikidata/28345908",
    name: "Apple Preferred",
    extensions: &["apf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
