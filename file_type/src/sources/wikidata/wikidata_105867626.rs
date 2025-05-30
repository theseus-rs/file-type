use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867626: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_626,
        source_type: SourceType::Wikidata,
        name: "Nintendo Switch Submission Zipped Package",
        extensions: &["nsz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x53, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
