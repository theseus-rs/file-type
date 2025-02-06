use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865321: FileFormat = FileFormat {
    id: 105_865_321,
    source_type: SourceType::Wikidata,
    name: "SMS Palette",
    extensions: &["pal"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x4C, 0x45, 0x54, 0x54, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
