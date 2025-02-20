use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855425: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_425,
        source_type: SourceType::Wikidata,
        name: "Flatpack Reference",
        extensions: &["flatpakref"],
        media_types: &["application/vnd.flatpak.ref"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x6C, 0x61, 0x74, 0x70, 0x61, 0x6B, 0x20, 0x52, 0x65, 0x66,
                        0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
