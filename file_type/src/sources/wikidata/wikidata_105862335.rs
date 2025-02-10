use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862335: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_335,
        source_type: SourceType::Wikidata,
        name: "dBASE 5.0 Multiple index",
        extensions: &["mdx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
