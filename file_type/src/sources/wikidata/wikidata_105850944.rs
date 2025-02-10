use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850944: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_944,
        source_type: SourceType::Wikidata,
        name: "FL Studio Touch Keyboard Form",
        extensions: &["tkp"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x6F, 0x75, 0x63, 0x68, 0x4B, 0x65, 0x79, 0x62, 0x46, 0x6F,
                        0x72, 0x6D, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
