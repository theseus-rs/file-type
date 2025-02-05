use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858880: FileFormat = FileFormat {
    id: 105_858_880,
    source_type: SourceType::Wikidata,
    name: "Grayscale BitMap",
    extensions: &["gbm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x42, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
