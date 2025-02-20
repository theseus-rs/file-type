use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863880: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_880,
        source_type: SourceType::Wikidata,
        name: "mzTab format",
        extensions: &["mztab"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x54, 0x44, 0x09, 0x6D, 0x7A, 0x54, 0x61, 0x62, 0x2D, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x09, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
