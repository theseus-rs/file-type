use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51917759: FileFormat = FileFormat {
    id: 51_917_759,
    source_type: SourceType::Wikidata,
    name: "SDSC Image Tool X Window Dump Format",
    extensions: &["xwd"],
    media_types: &["image/xwd"],
    signatures: &[],
    related_formats: &[],
};
