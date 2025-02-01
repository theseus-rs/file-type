use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859545: FileFormat = FileFormat {
    id: 105_859_545,
    puid: "wikidata/105859545",
    name: "Verilog source code (with rem 1)",
    extensions: &["v"],
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
