use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859108: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_108,
        source_type: SourceType::Wikidata,
        name: "Speccy eXtended Graphics bitmap",
        extensions: &["sxg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x53, 0x58, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
