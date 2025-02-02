use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129643497: FileFormat = FileFormat {
    id: 129_643_497,
    source_type: SourceType::Wikidata,
    name: "Icon file format",
    extensions: &["icon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
