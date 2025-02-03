use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206609: FileFormat = FileFormat {
    id: 28_206_609,
    source_type: SourceType::Wikidata,
    name: "Monochrome Recursive Format",
    extensions: &["mrf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x52, 0x46, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
