use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860102: FileFormat = FileFormat {
    id: 105_860_102,
    source_type: SourceType::Wikidata,
    name: "Verilog source code",
    extensions: &["v"],
    media_types: &["text/x-verilog"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
