use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_748,
        source_type: SourceType::Wikidata,
        name: "Ulead Imageiio/Imaginfo thumbnail",
        extensions: &["pe4"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x4F, 0x32, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
