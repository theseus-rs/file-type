use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857280: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_280,
        source_type: SourceType::Wikidata,
        name: "NoteCenter encrypted notes",
        extensions: &["hne"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4E, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
