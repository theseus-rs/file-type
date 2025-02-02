use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3928271: FileFormat = FileFormat {
    id: 3_928_271,
    source_type: SourceType::Wikidata,
    name: "RGBE image format",
    extensions: &["hdr", "pic", "rad", "rgbe", "xyze"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
