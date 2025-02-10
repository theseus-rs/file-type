use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856630: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_630,
        source_type: SourceType::Wikidata,
        name: "WinDev Index",
        extensions: &["ndx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x53, 0x00, 0x14, 0x00, 0x00, 0x00, 0x02, 0x00, 0x07, 0x00,
                        0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x10, 0x04,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
