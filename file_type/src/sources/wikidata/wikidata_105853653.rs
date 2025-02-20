use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_653,
        source_type: SourceType::Wikidata,
        name: "ABC notation (old)",
        extensions: &["abc"],
        media_types: &["text/vnd.abc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
