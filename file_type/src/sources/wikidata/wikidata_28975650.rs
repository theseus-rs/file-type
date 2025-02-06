use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975650: FileFormat = FileFormat {
    id: 28_975_650,
    source_type: SourceType::Wikidata,
    name: "Recon Mesh",
    extensions: &["m"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
