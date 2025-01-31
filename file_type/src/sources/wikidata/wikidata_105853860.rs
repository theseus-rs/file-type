use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853860: FileFormat = FileFormat {
    id: 105_853_860,
    puid: "wikidata/105853860",
    name: "Hamarsoft HAP compressed archive (v3.00)",
    extensions: &["hap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x91, 0x33, 0x48, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
