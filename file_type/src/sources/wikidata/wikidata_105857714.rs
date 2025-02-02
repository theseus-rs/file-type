use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857714: FileFormat = FileFormat {
    id: 105_857_714,
    source_type: SourceType::Wikidata,
    name: "DOSIMG disk image (80t/15s)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x0F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
