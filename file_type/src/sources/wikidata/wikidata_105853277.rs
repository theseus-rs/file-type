use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853277: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_277,
        source_type: SourceType::Wikidata,
        name: "Starquake high scores (MS-DOS)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6F, 0x72, 0x65, 0x20,
                        0x6F, 0x66, 0x20, 0x68, 0x65, 0x72, 0x6F, 0x65, 0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
