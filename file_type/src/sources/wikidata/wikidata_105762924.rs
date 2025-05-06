use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762924: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_924,
        source_type: SourceType::Wikidata,
        name: "X-Windows Screen Dump (X10)",
        extensions: &["xwd"],
        media_types: &["image/x-xwindowdump"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x06,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
