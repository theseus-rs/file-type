use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967194: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_194,
        source_type: SourceType::Wikidata,
        name: "Imago Orpheus module",
        extensions: &["imf"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4D, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
