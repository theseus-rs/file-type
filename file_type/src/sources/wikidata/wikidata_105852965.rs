use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852965: FileFormat = FileFormat {
    id: 105_852_965,
    puid: "wikidata/105852965",
    name: "Z64K Snapshot File",
    extensions: &["zsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x36, 0x34, 0x4B, 0x20, 0x53, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F, 0x74,
                    0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
