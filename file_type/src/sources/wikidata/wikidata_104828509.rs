use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104828509: FileFormat = FileFormat {
    id: 104_828_509,
    source_type: SourceType::Wikidata,
    name: "PCG",
    extensions: &["pcg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x47, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
