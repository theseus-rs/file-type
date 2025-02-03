use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21462816: FileFormat = FileFormat {
    id: 21_462_816,
    source_type: SourceType::Wikidata,
    name: "Android Secure encrypted file",
    extensions: &["asec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
