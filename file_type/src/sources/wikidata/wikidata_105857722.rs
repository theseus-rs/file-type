use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857722: FileFormat = FileFormat {
    id: 105_857_722,
    source_type: SourceType::Wikidata,
    name: "Bochs sparse disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCE, 0x8A, 0x46, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
