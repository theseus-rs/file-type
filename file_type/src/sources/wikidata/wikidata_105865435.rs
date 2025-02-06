use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865435: FileFormat = FileFormat {
    id: 105_865_435,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Layout",
    extensions: &["ply"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x50, 0x4C, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
