use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864845: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_845,
        source_type: SourceType::Wikidata,
        name: "Microsoft Windows Vista / 7 Prefetch data",
        extensions: &["pf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x17, 0x00, 0x00, 0x00, 0x53, 0x43, 0x43, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
