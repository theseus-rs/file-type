use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859464: FileFormat = FileFormat {
    id: 105_859_464,
    source_type: SourceType::Wikidata,
    name: "RM/Quest module",
    extensions: &["qrm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x4D, 0x2F, 0x51, 0x75, 0x65, 0x73, 0x74, 0x20, 0x4D, 0x6F, 0x64, 0x75,
                    0x6C, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
