use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355412: FileFormat = FileFormat {
    id: 111_355_412,
    puid: "wikidata/111355412",
    name: "Panasonic voice file",
    extensions: &["vm1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x6E, 0x61, 0x73, 0x6F, 0x6E, 0x69, 0x63, 0x20, 0x53, 0x44, 0x20,
                    0x56, 0x6F, 0x69, 0x63, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
