use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_674,
        source_type: SourceType::Wikidata,
        name: "Smacker movie/video (newer)",
        extensions: &["smk"],
        media_types: &["video/vnd.radgamettools.smacker"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x4B, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
