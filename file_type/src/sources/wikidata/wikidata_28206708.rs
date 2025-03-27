use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206708: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_708,
        source_type: SourceType::Wikidata,
        name: "Portable Pixmap, binary variant",
        extensions: &["pnm", "ppm"],
        media_types: &["image/x-portable-anymap", "image/x-portable-pixmap"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x36])],
                },
            }],
        }],
        related_formats: &[],
    },
};
