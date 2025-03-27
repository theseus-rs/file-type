use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_223535: FileType = FileType {
    file_format: &FileFormat {
        id: 223_535,
        source_type: SourceType::Wikidata,
        name: "Matroska",
        extensions: &["mk3d", "mka", "mks", "mkv"],
        media_types: &["application/x-matroska"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3])],
                },
            }],
        }],
        related_formats: &[],
    },
};
