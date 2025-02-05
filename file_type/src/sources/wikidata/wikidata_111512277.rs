use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111512277: FileFormat = FileFormat {
    id: 111_512_277,
    source_type: SourceType::Wikidata,
    name: "ASEG-GDF2 Description file",
    extensions: &["des"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
