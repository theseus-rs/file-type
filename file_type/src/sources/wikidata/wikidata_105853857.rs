use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853857: FileFormat = FileFormat {
    id: 105_853_857,
    puid: "wikidata/105853857",
    name: "ArcSoft UI",
    extensions: &["aui"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x55, 0x49, 0x00, 0x00, 0x01, 0x00, 0x00, 0x41, 0x72, 0x63, 0x73, 0x6F,
                    0x66, 0x74, 0x20, 0x55, 0x49, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
