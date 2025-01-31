use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125134301: FileFormat = FileFormat {
    id: 125_134_301,
    puid: "wikidata/125134301",
    name: "YAM Addressbook",
    extensions: &["addressbook"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
