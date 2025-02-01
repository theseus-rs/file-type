use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859561: FileFormat = FileFormat {
    id: 105_859_561,
    puid: "wikidata/105859561",
    name: "Vox Proxy Macro",
    extensions: &["vpm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x5D, 0x0D, 0x0A, 0x6B, 0x65, 0x79,
                    0x31, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
