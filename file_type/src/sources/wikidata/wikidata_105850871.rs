use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850871: FileFormat = FileFormat {
    id: 105_850_871,
    source_type: SourceType::Wikidata,
    name: "SSH Key Revocation List",
    extensions: &["krl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x53, 0x48, 0x4B, 0x52, 0x4C, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
