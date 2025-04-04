use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_142,
        source_type: SourceType::Wikidata,
        name: "Madagascar: Escape 2 Africa game data archive",
        extensions: &["arc", "bld"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x47, 0x41, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
