use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851048: FileFormat = FileFormat {
    id: 105_851_048,
    puid: "wikidata/105851048",
    name: "Cargo manifest",
    extensions: &["toml"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
