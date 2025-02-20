use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857536: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_536,
        source_type: SourceType::Wikidata,
        name: "Xexor disk image",
        extensions: &["arc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
