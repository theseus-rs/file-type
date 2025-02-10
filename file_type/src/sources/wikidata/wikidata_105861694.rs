use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861694: FileFormat = FileFormat {
    id: 105_861_694,
    source_type: SourceType::Wikidata,
    name: "MicroStation Modification resource file",
    extensions: &["m01", "p01", "r01", "s01"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x6F, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                    0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
