use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857680: FileFormat = FileFormat {
    id: 105_857_680,
    source_type: SourceType::Wikidata,
    name: "Copy On Write disk image",
    extensions: &["cow"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x45, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
