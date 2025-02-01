use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853198: FileFormat = FileFormat {
    id: 105_853_198,
    puid: "wikidata/105853198",
    name: "BeepFX Sound Effects Project (v2)",
    extensions: &["spj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x6F, 0x75, 0x6E, 0x64, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x73, 0x50,
                    0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x56, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
