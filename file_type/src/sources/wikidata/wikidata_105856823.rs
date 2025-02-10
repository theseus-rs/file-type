use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_823,
        source_type: SourceType::Wikidata,
        name: "SubZero Game File map",
        extensions: &["gmf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x11, 0x53, 0x75, 0x62, 0x5A, 0x65, 0x72, 0x6F, 0x20, 0x47, 0x61, 0x6D,
                        0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
