use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864012: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_012,
        source_type: SourceType::Wikidata,
        name: "D-Flow FM Model Data",
        extensions: &["mdu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
