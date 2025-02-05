use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861930: FileFormat = FileFormat {
    id: 105_861_930,
    source_type: SourceType::Wikidata,
    name: "Advanced Layouter model",
    extensions: &["mus"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x58, 0x46, 0x6D, 0x75, 0x73, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
