use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_137,
        source_type: SourceType::Wikidata,
        name: "PERQemu Hard Disk image",
        extensions: &["phd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x45, 0x52, 0x51])],
                },
            }],
        }],
        related_formats: &[],
    },
};
