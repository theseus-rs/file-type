use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850059: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_059,
        source_type: SourceType::Wikidata,
        name: "Windows Clipboard (BK)",
        extensions: &["clp"],
        media_types: &["application/windows-clipboard"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0xC3])],
                },
            }],
        }],
        related_formats: &[],
    },
};
