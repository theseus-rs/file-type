use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867491: FileFormat = FileFormat {
    id: 105_867_491,
    source_type: SourceType::Wikidata,
    name: "Nintendo Switch Submission Package",
    extensions: &["nsp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x53, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
