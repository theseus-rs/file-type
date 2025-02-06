use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849729: FileFormat = FileFormat {
    id: 105_849_729,
    source_type: SourceType::Wikidata,
    name: "PPrint Color map",
    extensions: &["col"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4C, 0x4D, 0x41, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
