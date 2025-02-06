use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860773: FileFormat = FileFormat {
    id: 105_860_773,
    source_type: SourceType::Wikidata,
    name: "REKO PC cardset",
    extensions: &["rkp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x52, 0x45, 0x4B, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
