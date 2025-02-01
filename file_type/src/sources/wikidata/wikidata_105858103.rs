use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858103: FileFormat = FileFormat {
    id: 105_858_103,
    puid: "wikidata/105858103",
    name: "Xilinx iMPACT Programming File (JEDEC)",
    extensions: &["ipf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x65, 0x64, 0x65, 0x63, 0x43, 0x68, 0x61, 0x69, 0x6E, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
