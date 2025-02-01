use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855772: FileFormat = FileFormat {
    id: 105_855_772,
    puid: "wikidata/105855772",
    name: "Panorama Digital Elevation Model Settings",
    extensions: &["demset"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x45, 0x4D, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
