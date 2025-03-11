use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762835: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_835,
        source_type: SourceType::Wikidata,
        name: "Acu4GL/AcuCOBOL Extended File Descriptor",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
