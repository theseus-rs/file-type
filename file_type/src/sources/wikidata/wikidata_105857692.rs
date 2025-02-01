use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857692: FileFormat = FileFormat {
    id: 105_857_692,
    puid: "wikidata/105857692",
    name: "Archimedes Protected Disk image (unzipped)",
    extensions: &["adp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x50, 0x44, 0x58, 0x30, 0x30, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
