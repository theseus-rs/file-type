use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856559: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_559,
        source_type: SourceType::Wikidata,
        name: "Alpha Five Web Project Settings",
        extensions: &["wcp_settings"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x61, 0x74, 0x65, 0x67, 0x6F, 0x72, 0x79, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
