use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_535473: FileType = FileType {
    file_format: &FileFormat {
        id: 535_473,
        source_type: SourceType::Wikidata,
        name: "PCX file format family",
        extensions: &["pcc", "pcx"],
        media_types: &["image/vnd.zbrush.pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
