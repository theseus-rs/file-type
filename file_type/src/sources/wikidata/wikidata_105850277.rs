use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850277: FileFormat = FileFormat {
    id: 105_850_277,
    source_type: SourceType::Wikidata,
    name: "16bit COM LHarc SFX archive executable",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0x60, 0x19, 0x0B, 0x20, 0x24, 0x4C, 0x48, 0x61, 0x72, 0x63, 0x27, 0x73,
                    0x20, 0x53, 0x46, 0x58, 0x20, 0x31, 0x2E, 0x31, 0x33, 0x53, 0x20, 0x28, 0x63,
                    0x29, 0x59, 0x6F, 0x73, 0x68, 0x69, 0x2C, 0x20, 0x31, 0x39, 0x38, 0x39, 0x2E,
                    0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
