use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852031: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_031,
        source_type: SourceType::Wikidata,
        name: "MAME Save State",
        extensions: &["sta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x4D, 0x45, 0x53, 0x41, 0x56, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
