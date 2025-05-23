use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_618,
        source_type: SourceType::Wikidata,
        name: "Z80 music code with AY music",
        extensions: &["aym"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x59, 0x4D, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
