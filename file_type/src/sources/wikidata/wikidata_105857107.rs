use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857107: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_107,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto 5 game data (generic)",
        extensions: &["ybn", "ydd", "ydr", "yft", "ymap", "ymt", "ytyp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x43, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
