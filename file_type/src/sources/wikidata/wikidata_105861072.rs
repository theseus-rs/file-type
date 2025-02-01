use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861072: FileFormat = FileFormat {
    id: 105_861_072,
    puid: "wikidata/105861072",
    name: "Linden Lab Structured Data",
    extensions: &["llsd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x6C, 0x6C, 0x73, 0x64, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
