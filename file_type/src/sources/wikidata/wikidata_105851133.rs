use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851133: FileFormat = FileFormat {
    id: 105_851_133,
    puid: "wikidata/105851133",
    name: "3D Ultra Cool data file",
    extensions: &["tbv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x42, 0x56, 0x6F, 0x6C, 0x75, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
