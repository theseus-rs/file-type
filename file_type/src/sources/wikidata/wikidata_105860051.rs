use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860051: FileFormat = FileFormat {
    id: 105_860_051,
    source_type: SourceType::Wikidata,
    name: "QMovie Video",
    extensions: &["qmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4D, 0x56, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
