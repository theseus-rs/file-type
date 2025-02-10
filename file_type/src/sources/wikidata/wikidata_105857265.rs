use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857265: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_265,
        source_type: SourceType::Wikidata,
        name: "Hollywood Applet",
        extensions: &["hwa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x57, 0x5A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
