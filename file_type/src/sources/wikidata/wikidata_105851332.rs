use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851332: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_332,
        source_type: SourceType::Wikidata,
        name: "U.S. Navy Fighters Theater game data",
        extensions: &["t2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x49, 0x54, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
