use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850936: FileFormat = FileFormat {
    id: 105_850_936,
    source_type: SourceType::Wikidata,
    name: "Character Table Library",
    extensions: &["tlb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x20, 0x54, 0x61, 0x62,
                    0x6C, 0x65, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
