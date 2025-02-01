use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862160: FileFormat = FileFormat {
    id: 105_862_160,
    puid: "wikidata/105862160",
    name: "Mednafen movie capture",
    extensions: &["mcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x44, 0x46, 0x4E, 0x4D, 0x4F, 0x56, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
