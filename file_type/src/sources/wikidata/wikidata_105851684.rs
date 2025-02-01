use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851684: FileFormat = FileFormat {
    id: 105_851_684,
    puid: "wikidata/105851684",
    name: "Microsoft Zoo Tycoon saved game",
    extensions: &["zoo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x5A, 0x46, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
