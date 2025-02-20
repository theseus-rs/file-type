use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859148: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_148,
        source_type: SourceType::Wikidata,
        name: "Artlantis BillBoard",
        extensions: &["bb"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x41, 0x62, 0x76, 0x65, 0x6E, 0x74, 0x2E, 0x42, 0x69, 0x6C, 0x6C,
                        0x62, 0x6F, 0x61, 0x72, 0x64, 0x3E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
