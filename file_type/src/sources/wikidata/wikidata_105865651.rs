use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865651: FileFormat = FileFormat {
    id: 105_865_651,
    source_type: SourceType::Wikidata,
    name: "Print Magic Font",
    extensions: &["pmf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x46, 0x4F, 0x4E, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
