use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861915: FileFormat = FileFormat {
    id: 105_861_915,
    puid: "wikidata/105861915",
    name: "Viewpoint MetaStream scene",
    extensions: &["mtx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x54, 0x53, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x20, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
