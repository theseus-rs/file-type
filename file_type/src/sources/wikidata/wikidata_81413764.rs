use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81413764: FileFormat = FileFormat {
    id: 81_413_764,
    source_type: SourceType::Wikidata,
    name: "EnCase Case data",
    extensions: &["cas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5F, 0x43, 0x41, 0x53, 0x45, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
