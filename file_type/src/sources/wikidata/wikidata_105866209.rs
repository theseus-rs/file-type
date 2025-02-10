use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_209,
        source_type: SourceType::Wikidata,
        name: "PC-BSD Installer Package",
        extensions: &["pbi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
