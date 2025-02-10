use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_187,
        source_type: SourceType::Wikidata,
        name: "Bitware BitFax page(s)",
        extensions: &["bfx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x54, 0x20, 0x20, 0x46, 0x41, 0x58, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
