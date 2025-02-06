use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860937: FileFormat = FileFormat {
    id: 105_860_937,
    source_type: SourceType::Wikidata,
    name: "RichView Format (Unicode)",
    extensions: &["rvf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x38, 0x20, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
