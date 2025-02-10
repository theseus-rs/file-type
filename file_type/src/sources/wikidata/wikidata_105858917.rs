use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_917,
        source_type: SourceType::Wikidata,
        name: "QuickTime Image Format bitmap",
        extensions: &["qif", "qtif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x64, 0x61, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
