use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865085: FileFormat = FileFormat {
    id: 105_865_085,
    puid: "wikidata/105865085",
    name: "ProSpace schematic",
    extensions: &["psa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x52, 0x4F, 0x53, 0x50, 0x41, 0x43, 0x45, 0x20, 0x53, 0x43, 0x48, 0x45,
                    0x4D, 0x41, 0x54, 0x49, 0x43, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x0D, 0x0A, 0x3B,
                    0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
