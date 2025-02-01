use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855649: FileFormat = FileFormat {
    id: 105_855_649,
    puid: "wikidata/105855649",
    name: "OrCAD Project",
    extensions: &["opj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x45, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                    0x63, 0x74, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
