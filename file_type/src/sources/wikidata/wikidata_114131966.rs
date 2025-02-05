use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114131966: FileFormat = FileFormat {
    id: 114_131_966,
    source_type: SourceType::Wikidata,
    name: "Chem3D template",
    extensions: &["c3t"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
