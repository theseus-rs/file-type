use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863435: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_435,
        source_type: SourceType::Wikidata,
        name: "Extensible Binary Meta Language / Matroska stream",
        extensions: &["mka", "mkv"],
        media_types: &["application/octet-stream"],
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
