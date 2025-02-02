use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48692391: FileFormat = FileFormat {
    id: 48_692_391,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 2002",
    extensions: &["vsd"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
