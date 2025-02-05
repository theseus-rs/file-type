use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975912: FileFormat = FileFormat {
    id: 28_975_912,
    source_type: SourceType::Wikidata,
    name: "XGL",
    extensions: &["xgl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
