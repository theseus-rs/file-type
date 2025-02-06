use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856315: FileFormat = FileFormat {
    id: 105_856_315,
    source_type: SourceType::Wikidata,
    name: "Data Doctor Recovery NTFS data",
    extensions: &["dnt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x20, 0x44, 0x6F, 0x63, 0x74, 0x6F, 0x72, 0x20, 0x52,
                    0x65, 0x63, 0x6F, 0x76, 0x65, 0x72, 0x79, 0x20, 0x4E, 0x54, 0x46, 0x53, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
