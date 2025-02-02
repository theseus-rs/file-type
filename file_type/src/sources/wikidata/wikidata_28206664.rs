use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206664: FileFormat = FileFormat {
    id: 28_206_664,
    source_type: SourceType::Wikidata,
    name: "Nero CoverDesigner Template",
    extensions: &["nct"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x56, 0x45, 0x52, 0x20, 0x45, 0x44, 0x49, 0x54, 0x4F, 0x52, 0x11,
                    0x09, 0x19, 0x77, 0xD5, 0xBF, 0x41, 0x37, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
                    0x6C, 0x5B, 0x01, 0x01, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
