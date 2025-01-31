use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864075: FileFormat = FileFormat {
    id: 105_864_075,
    puid: "wikidata/105864075",
    name: "Scrull Music File",
    extensions: &["smf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x63, 0x72, 0x75, 0x6C, 0x6C, 0x20, 0x6D, 0x75, 0x73, 0x69, 0x63, 0x20,
                    0x66, 0x69, 0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
