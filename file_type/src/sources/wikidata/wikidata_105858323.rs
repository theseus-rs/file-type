use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_323,
        source_type: SourceType::Wikidata,
        name: "EasyPrint Preview",
        extensions: &["epp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x41, 0x53, 0x59, 0x50, 0x52, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x45,
                        0x56, 0x49, 0x45, 0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
