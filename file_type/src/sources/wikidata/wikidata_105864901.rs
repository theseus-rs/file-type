use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864901: FileFormat = FileFormat {
    id: 105_864_901,
    source_type: SourceType::Wikidata,
    name: "OS/2 Pointer (bi-level)",
    extensions: &["ptr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x54, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
