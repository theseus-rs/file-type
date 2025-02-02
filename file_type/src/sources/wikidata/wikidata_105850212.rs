use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850212: FileFormat = FileFormat {
    id: 105_850_212,
    source_type: SourceType::Wikidata,
    name: "AmiDock Configuration",
    extensions: &["config"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
