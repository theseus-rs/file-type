use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849824: FileFormat = FileFormat {
    id: 105_849_824,
    source_type: SourceType::Wikidata,
    name: "Color Correction Collection",
    extensions: &["ccc"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x43, 0x6F, 0x72, 0x72, 0x65, 0x63, 0x74,
                    0x69, 0x6F, 0x6E, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
