use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855090: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_090,
        source_type: SourceType::Wikidata,
        name: "TorrentZip compressed archive",
        extensions: &["zip"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x02, 0x00, 0x08, 0x00, 0x00, 0xBC,
                        0x98, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
