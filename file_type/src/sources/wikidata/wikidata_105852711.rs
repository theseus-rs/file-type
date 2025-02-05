use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852711: FileFormat = FileFormat {
    id: 105_852_711,
    source_type: SourceType::Wikidata,
    name: "GEMPACK data management info",
    extensions: &["sfc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x45, 0x4D, 0x50, 0x41, 0x4B, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x4D,
                    0x41, 0x4E, 0x41, 0x47, 0x45, 0x4D, 0x45, 0x4E, 0x54, 0x20, 0x46, 0x49, 0x4C,
                    0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
