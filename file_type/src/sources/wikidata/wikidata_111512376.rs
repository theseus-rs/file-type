use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111512376: FileFormat = FileFormat {
    id: 111_512_376,
    source_type: SourceType::Wikidata,
    name: "ASEG-GDF2- Data definition file",
    extensions: &["dfn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
