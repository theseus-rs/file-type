use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859564: FileFormat = FileFormat {
    id: 105_859_564,
    source_type: SourceType::Wikidata,
    name: "The Complete Animator Film video",
    extensions: &["tca"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x45, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
