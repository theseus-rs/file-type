use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849731: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_731,
        source_type: SourceType::Wikidata,
        name: "Chromium WebView cache",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC3, 0xCA, 0x04, 0xC1])],
                },
            }],
        }],
        related_formats: &[],
    },
};
