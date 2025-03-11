use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762953: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_953,
        source_type: SourceType::Wikidata,
        name: "Apocalyptica game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x53, 0x46, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
