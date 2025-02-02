use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861991: FileFormat = FileFormat {
    id: 105_861_991,
    source_type: SourceType::Wikidata,
    name: "MOdule (play)List",
    extensions: &["mol"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x4F, 0x52, 0x49, 0x45, 0x53, 0x5D,
                    0x0D, 0x0A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
