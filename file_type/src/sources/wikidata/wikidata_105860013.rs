use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860013: FileFormat = FileFormat {
    id: 105_860_013,
    puid: "wikidata/105860013",
    name: "VocalTec Media Descriptor",
    extensions: &["vmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x56, 0x4D, 0x44, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
