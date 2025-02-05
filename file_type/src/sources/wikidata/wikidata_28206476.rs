use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206476: FileFormat = FileFormat {
    id: 28_206_476,
    source_type: SourceType::Wikidata,
    name: "Kolor Raw",
    extensions: &["kro"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x52, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
