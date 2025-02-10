use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854286: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_286,
        source_type: SourceType::Wikidata,
        name: "Microsoft ASP.NET Web Form",
        extensions: &["aspx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x25, 0x40, 0x20, 0x50, 0x61, 0x67, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
