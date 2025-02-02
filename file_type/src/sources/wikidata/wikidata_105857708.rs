use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857708: FileFormat = FileFormat {
    id: 105_857_708,
    source_type: SourceType::Wikidata,
    name: "F64 disk image",
    extensions: &["f64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
