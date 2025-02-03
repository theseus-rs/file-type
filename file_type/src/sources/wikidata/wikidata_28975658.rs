use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975658: FileFormat = FileFormat {
    id: 28_975_658,
    source_type: SourceType::Wikidata,
    name: "SketchUp skp",
    extensions: &["skb", "skp"],
    media_types: &["application/vnd.sketchup.skp"],
    internal_signatures: &[],
    related_formats: &[],
};
