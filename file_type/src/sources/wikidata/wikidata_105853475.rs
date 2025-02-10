use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853475: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_475,
        source_type: SourceType::Wikidata,
        name: "Zortrax Z-Code",
        extensions: &["zcode"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x43, 0x6F, 0x64, 0x65, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
