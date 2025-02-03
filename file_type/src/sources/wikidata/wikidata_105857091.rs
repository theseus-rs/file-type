use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857091: FileFormat = FileFormat {
    id: 105_857_091,
    source_type: SourceType::Wikidata,
    name: "Guile Object bytecode (big endian)",
    extensions: &["go"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4F, 0x4F, 0x46, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
