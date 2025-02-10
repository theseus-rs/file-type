use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853743: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_743,
        source_type: SourceType::Wikidata,
        name: "Cryo Interactive APC audio",
        extensions: &["apc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x52, 0x59, 0x4F, 0x5F, 0x41, 0x50, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
