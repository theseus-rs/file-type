use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857996: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_996,
        source_type: SourceType::Wikidata,
        name: "DIV Games Studio Font Source",
        extensions: &["ifs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x46, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
