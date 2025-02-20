use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854975: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_975,
        source_type: SourceType::Wikidata,
        name: "YBS compressed archive",
        extensions: &["ybs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x42, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
