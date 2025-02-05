use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919163: FileFormat = FileFormat {
    id: 28_919_163,
    source_type: SourceType::Wikidata,
    name: "Cult3D",
    extensions: &["cd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
