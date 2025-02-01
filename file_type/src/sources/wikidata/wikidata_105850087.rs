use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850087: FileFormat = FileFormat {
    id: 105_850_087,
    puid: "wikidata/105850087",
    name: "Clam Antivirus ByteCode signatures",
    extensions: &["cbc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x6C, 0x61, 0x6D, 0x42, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
