use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849603: FileFormat = FileFormat {
    id: 105_849_603,
    source_type: SourceType::Wikidata,
    name: "CWTool disk image (binary) (v2)",
    extensions: &["cwt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x77, 0x74, 0x6F, 0x6F, 0x6C, 0x20, 0x72, 0x61, 0x77, 0x20, 0x64, 0x61,
                    0x74, 0x61, 0x20, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
