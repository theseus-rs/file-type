use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857168: FileFormat = FileFormat {
    id: 105_857_168,
    puid: "wikidata/105857168",
    name: "Health Level-7 data (pipe delimited)",
    extensions: &["hl7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x48, 0x7C, 0x5E, 0x7E, 0x5C, 0x26, 0x7C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
