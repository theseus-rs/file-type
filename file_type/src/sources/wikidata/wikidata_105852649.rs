use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852649: FileFormat = FileFormat {
    id: 105_852_649,
    puid: "wikidata/105852649",
    name: "Symantec LiveState recovery image",
    extensions: &["sv2i"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x44, 0x4F, 0x20, 0x63, 0x6C, 0x73, 0x69, 0x64, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
