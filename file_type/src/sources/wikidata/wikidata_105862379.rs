use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862379: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_379,
        source_type: SourceType::Wikidata,
        name: "Personal Font Maker Macro",
        extensions: &["mcr"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x46, 0x4D, 0x20, 0x4D, 0x43, 0x52, 0x4F, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
