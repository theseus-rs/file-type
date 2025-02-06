use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967438: FileFormat = FileFormat {
    id: 27_967_438,
    source_type: SourceType::Wikidata,
    name: "Digital Picture Exchange, version 2",
    extensions: &["dpx"],
    media_types: &["image/x-dpx"],
    signatures: &[],
    related_formats: &[],
};
