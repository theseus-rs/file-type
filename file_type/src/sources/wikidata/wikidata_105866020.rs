use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_020,
        source_type: SourceType::Wikidata,
        name: "Playstation Portable Texture",
        extensions: &["ppt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x70, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
