use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866276: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_276,
        source_type: SourceType::Wikidata,
        name: "Bitmapped Signum!2 printer font (9 Pins)",
        extensions: &["p9"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x73, 0x30, 0x39, 0x30, 0x30, 0x30, 0x31, 0x00, 0x00, 0x00, 0x80,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
