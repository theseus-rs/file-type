use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863818: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_818,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help (old)",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x65, 0x87])],
                },
            }],
        }],
        related_formats: &[],
    },
};
