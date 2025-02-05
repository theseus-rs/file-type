use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919154: FileFormat = FileFormat {
    id: 28_919_154,
    source_type: SourceType::Wikidata,
    name: "Rhino 3D Model Backup",
    extensions: &["3dmbak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
