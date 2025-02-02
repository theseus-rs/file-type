use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858680: FileFormat = FileFormat {
    id: 105_858_680,
    source_type: SourceType::Wikidata,
    name: "Xilinx Block Memory Map (with rem)",
    extensions: &["bmm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
