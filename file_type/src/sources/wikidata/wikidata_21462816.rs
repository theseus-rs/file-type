use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21462816: FileFormat = FileFormat {
    id: 21_462_816,
    source_type: SourceType::Wikidata,
    name: "Android Secure encrypted file",
    extensions: &["asec"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
