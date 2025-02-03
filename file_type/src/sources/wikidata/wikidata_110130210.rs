use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110130210: FileFormat = FileFormat {
    id: 110_130_210,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 4",
    extensions: &["vsd", "vss", "vst", "vsw"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
