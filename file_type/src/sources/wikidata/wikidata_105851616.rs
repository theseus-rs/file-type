use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851616: FileFormat = FileFormat {
    id: 105_851_616,
    source_type: SourceType::Wikidata,
    name: "GPS track",
    extensions: &["twl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFF, 0x01, 0x00, 0x09, 0x00, 0x43, 0x57, 0x61, 0x79, 0x50, 0x6F, 0x69,
                    0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
