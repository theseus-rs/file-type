use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_814,
        source_type: SourceType::Wikidata,
        name: "Nintendo DS Packed Images",
        extensions: &["ipk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x63, 0x6B, 0x65, 0x64, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x73,
                        0x30, 0x31, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
