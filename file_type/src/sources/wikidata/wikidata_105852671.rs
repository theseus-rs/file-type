use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852671: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_671,
        source_type: SourceType::Wikidata,
        name: "Elecbyte M.U.G.E.N. sprites",
        extensions: &["sff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6C, 0x65, 0x63, 0x62, 0x79, 0x74, 0x65, 0x53, 0x70, 0x72, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
