use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7265393: FileFormat = FileFormat {
    id: 7_265_393,
    source_type: SourceType::Wikidata,
    name: "QCP",
    extensions: &["qcp"],
    media_types: &["audio/qcelp", "audio/vnd.qcelp"],
    internal_signatures: &[],
    related_formats: &[],
};
