use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852660: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_660,
        source_type: SourceType::Wikidata,
        name: "Microsoft App-V Sequencer SFT",
        extensions: &["sft"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
