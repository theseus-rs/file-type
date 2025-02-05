use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_93431491: FileFormat = FileFormat {
    id: 93_431_491,
    source_type: SourceType::Wikidata,
    name: "Final Draft Document 8",
    extensions: &["fdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
