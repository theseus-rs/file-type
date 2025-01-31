use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854446: FileFormat = FileFormat {
    id: 105_854_446,
    puid: "wikidata/105854446",
    name: "Advanced Renamer method",
    extensions: &["aren"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5D, 0x0D, 0x0A, 0x74, 0x79, 0x70,
                    0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
