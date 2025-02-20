use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_583,
        source_type: SourceType::Wikidata,
        name: "ArcSoft VideoImpression project",
        extensions: &["vi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x9D, 0xDB, 0xC5, 0x26, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
