use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861014: FileFormat = FileFormat {
    id: 105_861_014,
    puid: "wikidata/105861014",
    name: "LD Linker Script (with rem)",
    extensions: &["lds"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
