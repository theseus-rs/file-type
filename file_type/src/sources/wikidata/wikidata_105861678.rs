use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861678: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_678,
        source_type: SourceType::Wikidata,
        name: "Links LS game data archive",
        extensions: &["ani", "crx", "ls"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x53, 0x49, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
