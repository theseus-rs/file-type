use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855258: FileFormat = FileFormat {
    id: 105_855_258,
    puid: "wikidata/105855258",
    name: "PC Storyboard 1.0 Font",
    extensions: &["fac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x53, 0x50, 0x4D, 0x74, 0x79, 0x70, 0x65, 0x66, 0x61, 0x63, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
