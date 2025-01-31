use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854034: FileFormat = FileFormat {
    id: 105_854_034,
    puid: "wikidata/105854034",
    name: "PAQ8JC compressed archive",
    extensions: &["paq8jc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x61, 0x71, 0x38, 0x6A, 0x63, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
