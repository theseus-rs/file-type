use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855939: FileFormat = FileFormat {
    id: 105_855_939,
    puid: "wikidata/105855939",
    name: "Audio Interface Library 3 Digital audio driver",
    extensions: &["dig"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x49, 0x4C, 0x33, 0x44, 0x49, 0x47, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
