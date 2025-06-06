use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71264063: FileType = FileType {
    file_format: &FileFormat {
        id: 71_264_063,
        source_type: SourceType::Wikidata,
        name: "Hauptwerk copy-protected samples format",
        extensions: &["hbw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0x48, 0x50, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
