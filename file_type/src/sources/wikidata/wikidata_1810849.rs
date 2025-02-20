use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1810849: FileType = FileType {
    file_format: &FileFormat {
        id: 1_810_849,
        source_type: SourceType::Wikidata,
        name: "XLIFF",
        extensions: &["xlf"],
        media_types: &["application/xliff+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
