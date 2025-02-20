use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850602: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_602,
        source_type: SourceType::Wikidata,
        name: "NetCaptor's CaptorGroup",
        extensions: &["cgp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x61, 0x70, 0x74, 0x6F, 0x72, 0x47, 0x72, 0x6F, 0x75, 0x70,
                        0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
