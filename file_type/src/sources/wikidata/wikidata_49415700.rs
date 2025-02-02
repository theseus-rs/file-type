use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49415700: FileFormat = FileFormat {
    id: 49_415_700,
    source_type: SourceType::Wikidata,
    name: "CATIA file format, version 5",
    extensions: &["catmaterial"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
