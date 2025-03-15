use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122904901: FileType = FileType {
    file_format: &FileFormat {
        id: 122_904_901,
        source_type: SourceType::Wikidata,
        name: "PMTiles",
        extensions: &["pmtiles"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x54, 0x69, 0x6C, 0x65, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
