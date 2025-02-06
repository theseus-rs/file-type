use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855081: FileFormat = FileFormat {
    id: 105_855_081,
    source_type: SourceType::Wikidata,
    name: "Siva archive (generic)",
    extensions: &["siva"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
