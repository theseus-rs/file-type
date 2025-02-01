use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857361: FileFormat = FileFormat {
    id: 105_857_361,
    puid: "wikidata/105857361",
    name: "JNI Library",
    extensions: &["jnilib"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
