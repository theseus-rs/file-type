use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854891: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_891,
        source_type: SourceType::Wikidata,
        name: "HPACK compressed archive",
        extensions: &["hpk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x50, 0x41, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
