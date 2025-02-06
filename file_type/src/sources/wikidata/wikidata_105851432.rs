use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851432: FileFormat = FileFormat {
    id: 105_851_432,
    source_type: SourceType::Wikidata,
    name: "Captions Inc. subtitles",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x54, 0x69, 0x6D, 0x65, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x74, 0x79, 0x70,
                    0x65, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
