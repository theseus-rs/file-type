use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853702: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_702,
        source_type: SourceType::Wikidata,
        name: "rzip compressed archive",
        extensions: &["rz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x5A, 0x49, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
