use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856638: FileFormat = FileFormat {
    id: 105_856_638,
    puid: "wikidata/105856638",
    name: "WhatPulse Pre-Pulse Information",
    extensions: &["wpw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0xB8, 0x53, 0x3A, 0x08, 0x44, 0x9A, 0x40, 0x32, 0xED, 0x75, 0x27, 0xB3,
                    0x3E, 0xEF, 0x5C, 0xBD, 0xB8, 0x2E, 0xAC, 0xBD, 0x76, 0x20, 0x24, 0xEF, 0xD6,
                    0x00, 0xAD, 0x6C, 0x11, 0x1C, 0xC4, 0x9E, 0x32, 0xCD, 0x09, 0x86, 0x72, 0x80,
                    0xE5, 0x1C, 0x7F, 0x29, 0xD3, 0x37, 0x80, 0xEE, 0xEE,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
