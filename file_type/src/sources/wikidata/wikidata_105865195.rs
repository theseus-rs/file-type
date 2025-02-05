use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865195: FileFormat = FileFormat {
    id: 105_865_195,
    source_type: SourceType::Wikidata,
    name: "Snark Busters game data archive",
    extensions: &["pack"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x46, 0x50, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
