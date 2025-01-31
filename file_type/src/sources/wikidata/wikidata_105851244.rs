use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851244: FileFormat = FileFormat {
    id: 105_851_244,
    puid: "wikidata/105851244",
    name: "Borland Turbo Debugger session-state settings",
    extensions: &["tr2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x67, 0x65,
                    0x72, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
