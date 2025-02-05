use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858220: FileFormat = FileFormat {
    id: 105_858_220,
    source_type: SourceType::Wikidata,
    name: "Encore Musical Notation",
    extensions: &["enc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
