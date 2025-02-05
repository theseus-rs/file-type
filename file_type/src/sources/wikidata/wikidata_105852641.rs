use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852641: FileFormat = FileFormat {
    id: 105_852_641,
    source_type: SourceType::Wikidata,
    name: "Starry Night Document",
    extensions: &["snf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x48, 0x54, 0x4D, 0x4C, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
