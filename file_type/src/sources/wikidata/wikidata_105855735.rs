use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_735,
        source_type: SourceType::Wikidata,
        name: "TechSoft 2D Design drawing",
        extensions: &["dtd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x64, 0x74, 0x32, 0x64, 0x64, 0x74, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
