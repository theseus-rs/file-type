use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854089: FileFormat = FileFormat {
    id: 105_854_089,
    source_type: SourceType::Wikidata,
    name: "EPF game data archive",
    extensions: &["epf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x46, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
