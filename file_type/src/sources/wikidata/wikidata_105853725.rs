use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853725: FileFormat = FileFormat {
    id: 105_853_725,
    puid: "wikidata/105853725",
    name: "Winamp Advanced Visualization Studio File",
    extensions: &["avs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x75, 0x6C, 0x6C, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x41, 0x56, 0x53, 0x20,
                    0x50, 0x72, 0x65, 0x73, 0x65, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
