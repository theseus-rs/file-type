use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131471383: FileFormat = FileFormat {
    id: 131_471_383,
    source_type: SourceType::Wikidata,
    name: "Compressed MGH file format",
    extensions: &["mgh.gz", "mgz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
