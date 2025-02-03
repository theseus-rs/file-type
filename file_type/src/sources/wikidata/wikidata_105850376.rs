use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850376: FileFormat = FileFormat {
    id: 105_850_376,
    source_type: SourceType::Wikidata,
    name: "KGen98 Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x47, 0x45, 0x4E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
