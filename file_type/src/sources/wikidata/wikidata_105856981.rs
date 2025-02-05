use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856981: FileFormat = FileFormat {
    id: 105_856_981,
    source_type: SourceType::Wikidata,
    name: "Genstat Book",
    extensions: &["gwb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x57, 0x42, 0x01, 0x44, 0x42, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
