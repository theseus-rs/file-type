use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205742: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_742,
        source_type: SourceType::Wikidata,
        name: "Artweaver AWD",
        extensions: &["awd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
