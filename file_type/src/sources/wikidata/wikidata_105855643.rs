use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855643: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_643,
        source_type: SourceType::Wikidata,
        name: "OnlineTVRecorder (OTR) Keyfile",
        extensions: &["otrkey"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x54, 0x52, 0x4B, 0x45, 0x59, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
