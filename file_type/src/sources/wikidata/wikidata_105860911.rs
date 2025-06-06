use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_911,
        source_type: SourceType::Wikidata,
        name: "SpartaDOS X binary image",
        extensions: &["rom"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x44, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
