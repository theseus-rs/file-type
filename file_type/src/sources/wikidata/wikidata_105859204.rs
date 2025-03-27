use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859204: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_204,
        source_type: SourceType::Wikidata,
        name: "Drazpaint (C64) bitmap",
        extensions: &["drp", "drz"],
        media_types: &["image/x-draz-paint"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
