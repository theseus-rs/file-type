use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861047: FileFormat = FileFormat {
    id: 105_861_047,
    puid: "wikidata/105861047",
    name: "Jynx Snapshot",
    extensions: &["lynxsnapshot"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3E, 0x63, 0x61, 0x6D, 0x70, 0x75, 0x74, 0x65, 0x72, 0x73, 0x5F, 0x6C, 0x79,
                    0x6E, 0x78,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
