use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866130: FileFormat = FileFormat {
    id: 105_866_130,
    puid: "wikidata/105866130",
    name: "Psion serie 3 Application Alias",
    extensions: &["als"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x70, 0x70, 0x41, 0x6C, 0x69, 0x61, 0x73, 0x46, 0x69, 0x6C, 0x65, 0x2A,
                    0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
