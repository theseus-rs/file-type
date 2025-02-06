use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757993: FileFormat = FileFormat {
    id: 28_757_993,
    source_type: SourceType::Wikidata,
    name: "IWA",
    extensions: &["iwa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
