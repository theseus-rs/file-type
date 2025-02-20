use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861285: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_285,
        source_type: SourceType::Wikidata,
        name: "OpenTTD Language data",
        extensions: &["lng"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x41, 0x4E, 0x47, 0xCD, 0xED, 0x07, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
