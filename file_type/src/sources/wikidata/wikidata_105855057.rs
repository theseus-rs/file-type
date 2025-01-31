use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855057: FileFormat = FileFormat {
    id: 105_855_057,
    puid: "wikidata/105855057",
    name: "3D Studio ASCII format",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6D, 0x62, 0x69, 0x65, 0x6E, 0x74, 0x20, 0x6C, 0x69, 0x67, 0x68, 0x74,
                    0x20, 0x63, 0x6F, 0x6C, 0x6F, 0x72, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
