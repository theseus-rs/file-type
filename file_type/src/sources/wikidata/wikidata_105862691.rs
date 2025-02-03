use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862691: FileFormat = FileFormat {
    id: 105_862_691,
    source_type: SourceType::Wikidata,
    name: "MegaStation MIDI module",
    extensions: &["msm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x47, 0x4D, 0x30, 0x2E, 0x31, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
