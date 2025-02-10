use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858687: FileFormat = FileFormat {
    id: 105_858_687,
    source_type: SourceType::Wikidata,
    name: "Picture Packer bitmap",
    extensions: &["pp1", "pp2", "pp3"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x06, 0x07, 0x19, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
