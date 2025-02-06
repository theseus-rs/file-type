use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857532: FileFormat = FileFormat {
    id: 105_857_532,
    source_type: SourceType::Wikidata,
    name: "Indigo Renderer Material",
    extensions: &["igm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
