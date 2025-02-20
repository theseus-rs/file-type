use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850652: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_652,
        source_type: SourceType::Wikidata,
        name: "Gnome Keyring Store",
        extensions: &["keystore"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x6E, 0x6F, 0x6D, 0x65, 0x20, 0x4B, 0x65, 0x79, 0x72, 0x69, 0x6E,
                        0x67, 0x20, 0x53, 0x74, 0x6F, 0x72, 0x65, 0x20, 0x32, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
