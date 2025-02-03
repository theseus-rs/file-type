use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48692225: FileFormat = FileFormat {
    id: 48_692_225,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 2000",
    extensions: &["vsd"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
