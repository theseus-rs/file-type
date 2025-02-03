use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864236: FileFormat = FileFormat {
    id: 105_864_236,
    source_type: SourceType::Wikidata,
    name: "Panda multifile object",
    extensions: &["p3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x21, 0x20, 0x2F, 0x75, 0x73, 0x72, 0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x65,
                    0x6E, 0x76, 0x20, 0x70, 0x61, 0x6E, 0x64, 0x61, 0x33, 0x64, 0x0A, 0x70, 0x6D,
                    0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
