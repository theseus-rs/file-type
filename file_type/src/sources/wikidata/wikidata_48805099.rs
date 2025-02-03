use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48805099: FileFormat = FileFormat {
    id: 48_805_099,
    source_type: SourceType::Wikidata,
    name: "Btrieve Database",
    extensions: &["btr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
