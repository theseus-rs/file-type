use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857136: FileFormat = FileFormat {
    id: 105_857_136,
    source_type: SourceType::Wikidata,
    name: "CrossStudio project",
    extensions: &["hzp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x43, 0x72, 0x6F,
                    0x73, 0x73, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x5F, 0x50, 0x72, 0x6F, 0x6A,
                    0x65, 0x63, 0x74, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x3E, 0x0A, 0x3C, 0x73, 0x6F,
                    0x6C, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
