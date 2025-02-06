use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3928271: FileFormat = FileFormat {
    id: 3_928_271,
    source_type: SourceType::Wikidata,
    name: "RGBE image format",
    extensions: &["hdr", "pic", "rad", "rgbe", "xyze"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
