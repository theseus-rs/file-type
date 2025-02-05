use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76158565: FileFormat = FileFormat {
    id: 76_158_565,
    source_type: SourceType::Wikidata,
    name: "MegaPaint Vector symbols Library",
    extensions: &["vlb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x56, 0x4C, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
