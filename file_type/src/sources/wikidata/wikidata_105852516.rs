use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_516,
        source_type: SourceType::Wikidata,
        name: "Sound Effect Editor for PSG (original)",
        extensions: &["see"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x45, 0x33, 0x4F, 0x52, 0x47, 0x11,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
