use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850418: FileFormat = FileFormat {
    id: 105_850_418,
    source_type: SourceType::Wikidata,
    name: "CodeBox snippet library",
    extensions: &["cbxml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x73, 0x74, 0x61, 0x6E, 0x64, 0x61,
                    0x6C, 0x6F, 0x6E, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
