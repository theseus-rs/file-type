use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855179: FileFormat = FileFormat {
    id: 105_855_179,
    source_type: SourceType::Wikidata,
    name: "FMOD Audio Events",
    extensions: &["fev"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x45, 0x56, 0x31, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
