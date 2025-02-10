use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852221: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_221,
        source_type: SourceType::Wikidata,
        name: "EGrid32 form snippet",
        extensions: &["snl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x47, 0x33, 0x32, 0x46, 0x5F, 0x53, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
