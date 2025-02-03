use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858250: FileFormat = FileFormat {
    id: 105_858_250,
    source_type: SourceType::Wikidata,
    name: "Win32 Executable kkrunchy compressed",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x5A, 0x66, 0x61, 0x72, 0x62, 0x72, 0x61, 0x75, 0x73, 0x63, 0x68, 0x50,
                    0x45, 0x00, 0x00, 0x4C, 0x01, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
