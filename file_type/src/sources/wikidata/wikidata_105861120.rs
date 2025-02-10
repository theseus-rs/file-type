use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861120: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_120,
        source_type: SourceType::Wikidata,
        name: "SyncTERM dialing directory",
        extensions: &["lst"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09])],
                },
            }],
        }],
        related_formats: &[],
    },
};
