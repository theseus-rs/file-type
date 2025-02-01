use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855669: FileFormat = FileFormat {
    id: 105_855_669,
    puid: "wikidata/105855669",
    name: "Olympus digital camera RAW image (MMOR)",
    extensions: &["orf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x4F, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
