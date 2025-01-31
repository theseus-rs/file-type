use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851992: FileFormat = FileFormat {
    id: 105_851_992,
    puid: "wikidata/105851992",
    name: "Wataroo Save state",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x61, 0x74, 0x61, 0x72, 0x6F, 0x6F, 0x20, 0x3A, 0x20, 0x53, 0x61, 0x76,
                    0x65, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
