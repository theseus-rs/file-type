use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850600: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_600,
        source_type: SourceType::Wikidata,
        name: "Z.A.R. Navigator Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x61, 0x72, 0x20, 0x4E, 0x61, 0x76, 0x69, 0x67, 0x61, 0x74, 0x6F,
                        0x72, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
                        0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
