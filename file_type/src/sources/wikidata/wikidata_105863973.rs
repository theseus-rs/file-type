use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863973: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_973,
        source_type: SourceType::Wikidata,
        name: "Macro Express Macro",
        extensions: &["mex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x45, 0x00, 0x00, 0xBB, 0x0B, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
