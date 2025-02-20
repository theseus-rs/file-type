use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_786,
        source_type: SourceType::Wikidata,
        name: "Vaulty obscured",
        extensions: &["vdata"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x62, 0x73, 0x63, 0x75, 0x72, 0x65, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
