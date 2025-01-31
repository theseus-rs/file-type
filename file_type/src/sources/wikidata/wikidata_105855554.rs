use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855554: FileFormat = FileFormat {
    id: 105_855_554,
    puid: "wikidata/105855554",
    name: "OpenCTM 3D mesh",
    extensions: &["ctm"],
    media_types: &["application/x-ctm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x43, 0x54, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
