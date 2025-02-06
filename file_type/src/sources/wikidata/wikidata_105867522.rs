use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867522: FileFormat = FileFormat {
    id: 105_867_522,
    source_type: SourceType::Wikidata,
    name: "ESET Smart Security Quarantine Data",
    extensions: &["ndf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x51, 0x44, 0x46, 0xBB, 0x0B, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
