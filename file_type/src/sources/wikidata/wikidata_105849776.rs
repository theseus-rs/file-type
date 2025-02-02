use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849776: FileFormat = FileFormat {
    id: 105_849_776,
    source_type: SourceType::Wikidata,
    name: "Caligari TrueSpace Object",
    extensions: &["cob"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x6C, 0x69, 0x67, 0x61, 0x72, 0x69, 0x20, 0x56, 0x30, 0x30, 0x2E,
                    0x30, 0x31, 0x42, 0x4C, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
