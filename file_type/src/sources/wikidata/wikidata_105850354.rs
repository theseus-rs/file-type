use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850354: FileFormat = FileFormat {
    id: 105_850_354,
    source_type: SourceType::Wikidata,
    name: "Lotus 123 configuration (V1)",
    extensions: &["cnf"],
    media_types: &["application/vnd.lotus-1-2-3"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x01, 0x08])],
            },
        }],
    }],
    related_formats: &[],
};
