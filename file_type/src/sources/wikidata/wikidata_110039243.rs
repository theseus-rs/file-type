use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110039243: FileFormat = FileFormat {
    id: 110_039_243,
    source_type: SourceType::Wikidata,
    name: "XIFF (Xerox Image File Format), version 2",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[],
    related_formats: &[],
};
