use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118144950: FileFormat = FileFormat {
    id: 118_144_950,
    source_type: SourceType::Wikidata,
    name: "Serenade Symphony File",
    extensions: &["sph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
