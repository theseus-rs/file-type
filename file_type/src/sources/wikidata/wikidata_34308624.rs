use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34308624: FileType = FileType {
    file_format: &FileFormat {
        id: 34_308_624,
        source_type: SourceType::Wikidata,
        name: "Scribe manuscript",
        extensions: &["mss"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
