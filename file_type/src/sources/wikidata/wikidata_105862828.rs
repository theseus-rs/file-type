use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862828: FileFormat = FileFormat {
    id: 105_862_828,
    puid: "wikidata/105862828",
    name: "Mass Effect 3 Head Morph",
    extensions: &["me3headmorph"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x49, 0x42, 0x42, 0x45, 0x44, 0x4D, 0x41, 0x53, 0x53, 0x45, 0x46, 0x46,
                    0x45, 0x43, 0x54, 0x33, 0x48, 0x45, 0x41, 0x44, 0x4D, 0x4F, 0x52, 0x50, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
