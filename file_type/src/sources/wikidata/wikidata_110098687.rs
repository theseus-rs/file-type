use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110098687: FileFormat = FileFormat {
    id: 110_098_687,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 2",
    extensions: &["vsd", "vss", "vst"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
