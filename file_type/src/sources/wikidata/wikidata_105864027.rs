use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864027: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_027,
        source_type: SourceType::Wikidata,
        name: "3by5 Index",
        extensions: &["map"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x62, 0x79, 0x35, 0x20, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x20,
                        0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
