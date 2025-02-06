use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855781: FileFormat = FileFormat {
    id: 105_855_781,
    source_type: SourceType::Wikidata,
    name: "Anachronox game data archive",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x41, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
