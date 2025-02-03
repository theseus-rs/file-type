use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114131966: FileFormat = FileFormat {
    id: 114_131_966,
    source_type: SourceType::Wikidata,
    name: "Chem3D template",
    extensions: &["c3t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
