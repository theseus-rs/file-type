use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852742: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_742,
        source_type: SourceType::Wikidata,
        name: "Windows Shadow spooler (NT)",
        extensions: &["shd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x49, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
