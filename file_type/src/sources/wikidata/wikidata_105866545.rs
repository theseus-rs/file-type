use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866545: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_545,
        source_type: SourceType::Wikidata,
        name: "Binary Delta Compressed Patch",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x31, 0x39])],
                },
            }],
        }],
        related_formats: &[],
    },
};
