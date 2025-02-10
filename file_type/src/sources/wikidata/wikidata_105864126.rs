use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864126: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_126,
        source_type: SourceType::Wikidata,
        name: "Octalyser 8-channel STe/Falcon Module",
        extensions: &["mod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x38, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
