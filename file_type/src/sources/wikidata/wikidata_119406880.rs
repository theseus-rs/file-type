use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119406880: FileFormat = FileFormat {
    id: 119_406_880,
    source_type: SourceType::Wikidata,
    name: "ACT! Data File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
