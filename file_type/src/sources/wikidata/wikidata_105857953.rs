use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857953: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_953,
        source_type: SourceType::Wikidata,
        name: "HFS+ / Mac OS Extended disk image (HFSX)",
        extensions: &["dmg", "hfs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
