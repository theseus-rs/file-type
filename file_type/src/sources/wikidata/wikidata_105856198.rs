use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856198: FileFormat = FileFormat {
    id: 105_856_198,
    puid: "wikidata/105856198",
    name: "DipTrace Schematic",
    extensions: &["dch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x44, 0x54, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x0F, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
