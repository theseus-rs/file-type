use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857660: FileFormat = FileFormat {
    id: 105_857_660,
    puid: "wikidata/105857660",
    name: "divANS Intermediate Representation",
    extensions: &["ir"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
