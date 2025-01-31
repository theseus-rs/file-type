use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850133: FileFormat = FileFormat {
    id: 105_850_133,
    puid: "wikidata/105850133",
    name: "Calamus Vector Graphic",
    extensions: &["cvg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x43, 0x56, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
