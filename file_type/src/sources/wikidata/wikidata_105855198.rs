use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855198: FileFormat = FileFormat {
    id: 105_855_198,
    source_type: SourceType::Wikidata,
    name: "Assassin's Creed game data container (generic)",
    extensions: &[
        "ac1", "ac2", "ac3", "ac3mp", "ac3sp", "ac4", "ac4mp", "ac4sp", "acb", "acl", "acre",
        "acro", "acu",
    ],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x63, 0x69, 0x6D, 0x69, 0x74, 0x61, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
