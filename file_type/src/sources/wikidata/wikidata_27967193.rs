use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967193: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_193,
        source_type: SourceType::Wikidata,
        name: "Graoumf Tracker 2 module",
        extensions: &["gt2"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x54, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
