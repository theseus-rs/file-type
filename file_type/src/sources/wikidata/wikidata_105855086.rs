use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855086: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_086,
        source_type: SourceType::Wikidata,
        name: "SndTool sound/audio",
        extensions: &["sndt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4F, 0x55, 0x4E, 0x44, 0x1A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x5D, 0x00, 0x00,
                        0x0A, 0x00, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
