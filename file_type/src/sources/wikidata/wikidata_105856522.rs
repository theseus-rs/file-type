use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856522: FileFormat = FileFormat {
    id: 105_856_522,
    source_type: SourceType::Wikidata,
    name: "Land Of Legends game data archive",
    extensions: &["wad"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCE, 0xCA, 0xEF, 0xBE])],
            },
        }],
    }],
    related_formats: &[],
};
