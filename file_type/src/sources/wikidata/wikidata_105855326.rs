use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855326: FileFormat = FileFormat {
    id: 105_855_326,
    source_type: SourceType::Wikidata,
    name: "Farscape: The Game Animation data",
    extensions: &["fsa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x61, 0x72, 0x73, 0x63, 0x61, 0x70, 0x65, 0x20, 0x61, 0x6E, 0x69, 0x6D,
                    0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
