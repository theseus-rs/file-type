use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979386: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_386,
        source_type: SourceType::Wikidata,
        name: "Protected Interoperable File Format",
        extensions: &["piff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70, 0x70, 0x69, 0x66, 0x66,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
