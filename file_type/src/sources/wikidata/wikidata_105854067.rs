use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854067: FileFormat = FileFormat {
    id: 105_854_067,
    puid: "wikidata/105854067",
    name: "paq8o8 compressed archive",
    extensions: &["paq8o8"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x61, 0x71, 0x38, 0x6F, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
