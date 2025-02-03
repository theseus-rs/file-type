use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859374: FileFormat = FileFormat {
    id: 105_859_374,
    source_type: SourceType::Wikidata,
    name: "Zelda Classic Quest",
    extensions: &["qst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x65, 0x6C, 0x64, 0x61, 0x20, 0x43, 0x6C, 0x61, 0x73, 0x73, 0x69, 0x63,
                    0x20, 0x51, 0x75, 0x65, 0x73, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
