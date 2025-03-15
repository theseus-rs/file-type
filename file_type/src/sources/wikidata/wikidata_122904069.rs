use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122904069: FileType = FileType {
    file_format: &FileFormat {
        id: 122_904_069,
        source_type: SourceType::Wikidata,
        name: "MBTiles",
        extensions: &["mbtiles"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x42, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
