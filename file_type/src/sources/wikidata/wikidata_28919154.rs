use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919154: FileFormat = FileFormat {
    id: 28_919_154,
    source_type: SourceType::Wikidata,
    name: "Rhino 3D Model Backup",
    extensions: &["3dmbak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
