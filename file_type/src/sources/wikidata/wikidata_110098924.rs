use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110098924: FileFormat = FileFormat {
    id: 110_098_924,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 3",
    extensions: &["vsd", "vss", "vst"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[],
};
