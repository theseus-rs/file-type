use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854602: FileFormat = FileFormat {
    id: 105_854_602,
    source_type: SourceType::Wikidata,
    name: "FMOD Sample Bank format (generic)",
    extensions: &["fsb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
