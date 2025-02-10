use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855550: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_550,
        source_type: SourceType::Wikidata,
        name: "AWE Games game data container",
        extensions: &["omt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x4D, 0x46, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
