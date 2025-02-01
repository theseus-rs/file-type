use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850552: FileFormat = FileFormat {
    id: 105_850_552,
    puid: "wikidata/105850552",
    name: "KOMPAS Drawing (Unicode)",
    extensions: &["cdw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
