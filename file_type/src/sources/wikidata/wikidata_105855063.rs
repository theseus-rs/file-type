use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855063: FileFormat = FileFormat {
    id: 105_855_063,
    source_type: SourceType::Wikidata,
    name: "Axon Text File format",
    extensions: &["atf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
