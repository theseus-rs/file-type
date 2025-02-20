use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852970: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_970,
        source_type: SourceType::Wikidata,
        name: "Hi-MD Minidisc MPEG audio data container",
        extensions: &["hma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x50, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
