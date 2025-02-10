use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857675: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_675,
        source_type: SourceType::Wikidata,
        name: "InDesign Book",
        extensions: &["indb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x06, 0xED, 0xF5, 0xD8, 0x1D, 0x46, 0xE5, 0xBD, 0x31, 0xEF, 0xE7,
                        0xFE, 0x74, 0xB7, 0x1D, 0x42, 0x4F, 0x4F, 0x4B, 0x42, 0x4F, 0x4F, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
