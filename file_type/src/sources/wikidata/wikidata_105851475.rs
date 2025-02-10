use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851475: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_475,
        source_type: SourceType::Wikidata,
        name: "TI-99 TIFILES file image",
        extensions: &["tfi", "tifile", "tifiles"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x54, 0x49, 0x46, 0x49, 0x4C, 0x45, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
