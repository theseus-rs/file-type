use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850863: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_863,
        source_type: SourceType::Wikidata,
        name: "Kea Coloring Book page",
        extensions: &["kcx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x19, 0x83, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
