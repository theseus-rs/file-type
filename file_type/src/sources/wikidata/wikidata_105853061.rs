use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853061: FileFormat = FileFormat {
    id: 105_853_061,
    puid: "wikidata/105853061",
    name: "SimStructure project (v3.00)",
    extensions: &["sim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x69, 0x6D, 0x53, 0x74, 0x72, 0x30, 0x33, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
