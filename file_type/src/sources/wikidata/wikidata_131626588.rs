use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131626588: FileFormat = FileFormat {
    id: 131_626_588,
    source_type: SourceType::Wikidata,
    name: "FLUENT CFF file format",
    extensions: &["dat.h5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
