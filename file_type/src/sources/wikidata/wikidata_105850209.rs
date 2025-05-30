use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_209,
        source_type: SourceType::Wikidata,
        name: "MicroHof Code",
        extensions: &["cde"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x48, 0x20, 0x43, 0x4F, 0x44, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
