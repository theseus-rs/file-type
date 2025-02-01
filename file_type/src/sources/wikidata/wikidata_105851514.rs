use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851514: FileFormat = FileFormat {
    id: 105_851_514,
    puid: "wikidata/105851514",
    name: "T'SoundSystem Source",
    extensions: &["tss"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
