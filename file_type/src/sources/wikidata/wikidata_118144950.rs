use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118144950: FileFormat = FileFormat {
    id: 118_144_950,
    source_type: SourceType::Wikidata,
    name: "Serenade Symphony File",
    extensions: &["sph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
