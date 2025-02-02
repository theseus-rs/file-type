use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864832: FileFormat = FileFormat {
    id: 105_864_832,
    source_type: SourceType::Wikidata,
    name: "PCE Flux Image disk image",
    extensions: &["pfi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x49, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
