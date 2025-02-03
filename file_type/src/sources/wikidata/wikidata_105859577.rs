use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859577: FileFormat = FileFormat {
    id: 105_859_577,
    source_type: SourceType::Wikidata,
    name: "VICE Rom Set",
    extensions: &["vrs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x65, 0x72, 0x6E, 0x61, 0x6C, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x22, 0x6B,
                    0x65, 0x72, 0x6E, 0x61, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
