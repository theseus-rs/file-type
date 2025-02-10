use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857595: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_595,
        source_type: SourceType::Wikidata,
        name: "CopyControl I.C.A. disk image",
        extensions: &["ica"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x43, 0x4F, 0x4E, 0x54, 0x52, 0x4F, 0x4C, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
