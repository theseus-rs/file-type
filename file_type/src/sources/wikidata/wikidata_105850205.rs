use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850205: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_205,
        source_type: SourceType::Wikidata,
        name: "Atomulator Configuration",
        extensions: &["cfg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x65, 0x62, 0x75, 0x67, 0x5F, 0x6F, 0x6E, 0x5F, 0x62, 0x72, 0x6B,
                        0x20, 0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
