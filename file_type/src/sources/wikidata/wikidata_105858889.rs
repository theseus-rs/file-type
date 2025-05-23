use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858889: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_889,
        source_type: SourceType::Wikidata,
        name: "QuickLink II Fax bitmap (old)",
        extensions: &["qfx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x4C, 0x49, 0x49, 0x46, 0x41, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
