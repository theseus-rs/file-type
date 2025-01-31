use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857192: FileFormat = FileFormat {
    id: 105_857_192,
    puid: "wikidata/105857192",
    name: "HEC-HMS Metereologic model configuration",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x74, 0x65, 0x6F, 0x72, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
