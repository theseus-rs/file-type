use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110098924: FileFormat = FileFormat {
    id: 110_098_924,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 3",
    extensions: &["vsd", "vss", "vst"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
