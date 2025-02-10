use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852718: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_718,
        source_type: SourceType::Wikidata,
        name: "NGS orbital format SP3 (positions only)",
        extensions: &["sp3"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x61, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
