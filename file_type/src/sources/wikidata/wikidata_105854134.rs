use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854134: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_134,
        source_type: SourceType::Wikidata,
        name: "MAr compressed archive",
        extensions: &["mar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x72, 0x30, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
