use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111512376: FileFormat = FileFormat {
    id: 111_512_376,
    source_type: SourceType::Wikidata,
    name: "ASEG-GDF2- Data definition file",
    extensions: &["dfn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
