use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975654: FileFormat = FileFormat {
    id: 28_975_654,
    source_type: SourceType::Wikidata,
    name: "Recon Points",
    extensions: &["pts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
