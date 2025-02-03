use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538977: FileFormat = FileFormat {
    id: 47_538_977,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Template Menu File",
    extensions: &["mnu"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
