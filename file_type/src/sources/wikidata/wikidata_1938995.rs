use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1938995: FileType = FileType {
    file_format: &FileFormat {
        id: 1_938_995,
        source_type: SourceType::Wikidata,
        name: "iMelody",
        extensions: &["imy"],
        media_types: &["audio/imelody"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x49, 0x4D, 0x45, 0x4C, 0x4F, 0x44,
                        0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
