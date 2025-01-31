use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866379: FileFormat = FileFormat {
    id: 105_866_379,
    puid: "wikidata/105866379",
    name: "Ping Plotter Sample file",
    extensions: &["pp2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x50, 0x69, 0x6E, 0x67, 0x20, 0x50, 0x6C, 0x6F, 0x74, 0x74, 0x65, 0x72,
                    0x20, 0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
