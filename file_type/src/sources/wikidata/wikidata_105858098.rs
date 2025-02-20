use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_098,
        source_type: SourceType::Wikidata,
        name: "HP Logical Interchange Format disk image",
        extensions: &["lif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
