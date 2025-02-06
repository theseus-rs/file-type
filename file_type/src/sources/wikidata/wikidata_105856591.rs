use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856591: FileFormat = FileFormat {
    id: 105_856_591,
    source_type: SourceType::Wikidata,
    name: "WinGenea Data File",
    extensions: &["wdf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x47, 0x65, 0x6E, 0x65, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
