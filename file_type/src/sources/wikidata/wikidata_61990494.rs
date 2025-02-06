use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61990494: FileFormat = FileFormat {
    id: 61_990_494,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 2003-2010",
    extensions: &["vsd"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[],
};
