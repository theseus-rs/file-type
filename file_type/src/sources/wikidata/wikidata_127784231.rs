use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127784231: FileFormat = FileFormat {
    id: 127_784_231,
    source_type: SourceType::Wikidata,
    name: "Logtalk source file",
    extensions: &["lgt"],
    media_types: &["text/x-logtalk"],
    internal_signatures: &[],
    related_formats: &[],
};
