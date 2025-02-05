use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039243: FileFormat = FileFormat {
    id: 110_039_243,
    source_type: SourceType::Wikidata,
    name: "XIFF (Xerox Image File Format), version 2",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    signatures: &[],
    related_formats: &[],
};
