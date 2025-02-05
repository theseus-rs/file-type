use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857390: FileFormat = FileFormat {
    id: 105_857_390,
    source_type: SourceType::Wikidata,
    name: "EzDraw drawing",
    extensions: &["joy"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x5A, 0x44, 0x72, 0x61, 0x77, 0x20, 0x42, 0x79, 0x20, 0x45, 0x61, 0x73,
                    0x74, 0x65, 0x72, 0x6E, 0x20, 0x4C, 0x6F, 0x67, 0x69, 0x63, 0x2D, 0x46, 0x49,
                    0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
