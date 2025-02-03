use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851438: FileFormat = FileFormat {
    id: 105_851_438,
    source_type: SourceType::Wikidata,
    name: "Win994a cartdrige image",
    extensions: &["ticart"],
    media_types: &["TI-99/4A PC99 disk image"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x39, 0x39, 0x34, 0x61, 0x43, 0x61, 0x72, 0x74, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
