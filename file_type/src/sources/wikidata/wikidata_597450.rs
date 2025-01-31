use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_597450: FileFormat = FileFormat {
    id: 597_450,
    puid: "wikidata/597450",
    name: "Shorten",
    extensions: &["shn"],
    media_types: &["application/x-shorten"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x6A, 0x6B, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
