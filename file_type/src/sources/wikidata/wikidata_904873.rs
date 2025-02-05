use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_904873: FileFormat = FileFormat {
    id: 904_873,
    source_type: SourceType::Wikidata,
    name: "Cryptographic Message Syntax",
    extensions: &["cmsc"],
    media_types: &["application/cms"],
    signatures: &[],
    related_formats: &[],
};
