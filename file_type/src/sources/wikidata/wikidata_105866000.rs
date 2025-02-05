use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866000: FileFormat = FileFormat {
    id: 105_866_000,
    source_type: SourceType::Wikidata,
    name: "Kyocera PRESCRIBE printing language",
    extensions: &["prn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x52, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
