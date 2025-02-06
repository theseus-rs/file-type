use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857346: FileFormat = FileFormat {
    id: 105_857_346,
    source_type: SourceType::Wikidata,
    name: "Jacksum fingerprints",
    extensions: &["jacksum"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x61, 0x63, 0x6B, 0x73, 0x75, 0x6D, 0x3A, 0x20, 0x4D, 0x65, 0x74, 0x61,
                    0x2D, 0x49, 0x6E, 0x66, 0x6F, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
