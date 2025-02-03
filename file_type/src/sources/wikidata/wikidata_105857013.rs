use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857013: FileFormat = FileFormat {
    id: 105_857_013,
    source_type: SourceType::Wikidata,
    name: "GLBasic Font",
    extensions: &["glfont"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x46, 0x6F, 0x6E, 0x74, 0x53, 0x74, 0x79, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
