use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860325: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_325,
        source_type: SourceType::Wikidata,
        name: "Helios raw disk image",
        extensions: &["raw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x45, 0x4C, 0x49, 0x4F, 0x53, 0x20, 0x72, 0x61, 0x77, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
