use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855968: FileFormat = FileFormat {
    id: 105_855_968,
    source_type: SourceType::Wikidata,
    name: "PhoneTools Internal Graphic Format",
    extensions: &["dgr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x47, 0x46, 0x20, 0x2D, 0x20, 0x28, 0x63, 0x29, 0x20, 0x42, 0x56, 0x52,
                    0x50, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x38, 0x39,
                    0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
