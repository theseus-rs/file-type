use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850075: FileFormat = FileFormat {
    id: 105_850_075,
    source_type: SourceType::Wikidata,
    name: "Kerbal Space Program (KSP) spacecraft",
    extensions: &["craft"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x68, 0x69, 0x70, 0x20, 0x3D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
