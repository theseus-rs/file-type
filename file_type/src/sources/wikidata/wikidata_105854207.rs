use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854207: FileFormat = FileFormat {
    id: 105_854_207,
    puid: "wikidata/105854207",
    name: "SAPCAR CAR compressed archive",
    extensions: &["sar"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x43, 0x41, 0x52, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                    0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
