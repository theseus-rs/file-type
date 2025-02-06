use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975881: FileFormat = FileFormat {
    id: 28_975_881,
    source_type: SourceType::Wikidata,
    name: "SOLIDWORKS Part",
    extensions: &["sldprt"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
