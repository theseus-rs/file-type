use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_260180: FileType = FileType {
    file_format: &FileFormat {
        id: 260_180,
        source_type: SourceType::Wikidata,
        name: "OpenType Font",
        extensions: &["otc", "otf", "ttc", "ttf"],
        media_types: &["font/otf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x54, 0x54, 0x4F, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
