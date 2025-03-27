use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50374896: FileType = FileType {
    file_format: &FileFormat {
        id: 50_374_896,
        source_type: SourceType::Wikidata,
        name: "Jamcracker Tracker Module",
        extensions: &["jam"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x45, 0x70, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
