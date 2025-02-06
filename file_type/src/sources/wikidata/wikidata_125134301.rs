use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125134301: FileFormat = FileFormat {
    id: 125_134_301,
    source_type: SourceType::Wikidata,
    name: "YAM Addressbook",
    extensions: &["addressbook"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
