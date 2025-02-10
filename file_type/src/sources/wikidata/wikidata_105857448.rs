use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857448: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_448,
        source_type: SourceType::Wikidata,
        name: "3D-Calc spreadsheet",
        extensions: &["3dd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x18, 0xB5])],
                },
            }],
        }],
        related_formats: &[],
    },
};
