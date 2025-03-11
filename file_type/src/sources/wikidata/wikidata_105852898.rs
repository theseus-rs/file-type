use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_898,
        source_type: SourceType::Wikidata,
        name: "Symantech Antivirus update",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x59, 0x4D, 0x50, 0x41, 0x54, 0x43, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
