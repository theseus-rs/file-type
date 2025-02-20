use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851442: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_442,
        source_type: SourceType::Wikidata,
        name: "TrueType/OpenType Font Collection (generic)",
        extensions: &["otc", "ttc"],
        media_types: &["font/collection"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x74, 0x63, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
