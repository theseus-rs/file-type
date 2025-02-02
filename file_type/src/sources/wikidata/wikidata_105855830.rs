use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855830: FileFormat = FileFormat {
    id: 105_855_830,
    source_type: SourceType::Wikidata,
    name: "R4 cheat codes",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x34, 0x20, 0x43, 0x68, 0x65, 0x61, 0x74, 0x43, 0x6F, 0x64, 0x65, 0x00,
                    0x01, 0x00, 0x00, 0x55, 0x73, 0x65, 0x72, 0x20, 0x63, 0x68, 0x65, 0x61, 0x74,
                    0x20, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x76, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
