use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865685: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_685,
        source_type: SourceType::Wikidata,
        name: "Palm iSilo 1.x/2.x document",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x6F, 0x47, 0x6F, 0x54, 0x6F, 0x47, 0x6F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
