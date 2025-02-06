use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959878: FileFormat = FileFormat {
    id: 27_959_878,
    source_type: SourceType::Wikidata,
    name: "Piston Collage song",
    extensions: &["ptcop", "pttune"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
