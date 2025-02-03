use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_283579: FileFormat = FileFormat {
    id: 283_579,
    source_type: SourceType::Wikidata,
    name: "tar",
    extensions: &["tar"],
    media_types: &["application/x-tar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x75, 0x73, 0x74, 0x61, 0x72])],
            },
        }],
    }],
    related_formats: &[],
};
