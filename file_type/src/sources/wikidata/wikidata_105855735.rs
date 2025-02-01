use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855735: FileFormat = FileFormat {
    id: 105_855_735,
    puid: "wikidata/105855735",
    name: "TechSoft 2D Design drawing",
    extensions: &["dtd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x64, 0x74, 0x32, 0x64, 0x64, 0x74, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
