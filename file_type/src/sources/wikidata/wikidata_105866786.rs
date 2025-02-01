use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866786: FileFormat = FileFormat {
    id: 105_866_786,
    puid: "wikidata/105866786",
    name: "ProfiCAD symbols (v4)",
    extensions: &["ppd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x43, 0x41, 0x44, 0x20, 0x50,
                    0x61, 0x72, 0x74, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
                    0x6F, 0x6E, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
