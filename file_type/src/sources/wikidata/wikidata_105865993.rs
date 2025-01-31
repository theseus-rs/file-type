use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865993: FileFormat = FileFormat {
    id: 105_865_993,
    puid: "wikidata/105865993",
    name: "PiXCL Binary Palette",
    extensions: &["pbl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x69, 0x58, 0x43, 0x4C, 0x2D, 0x50, 0x42, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
