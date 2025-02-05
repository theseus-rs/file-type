use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863906: FileFormat = FileFormat {
    id: 105_863_906,
    source_type: SourceType::Wikidata,
    name: "Gen Surf map",
    extensions: &["map"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
