use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650302: FileFormat = FileFormat {
    id: 29_650_302,
    source_type: SourceType::Wikidata,
    name: "Perfect ZX Tape",
    extensions: &["pzx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x5A, 0x58, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
