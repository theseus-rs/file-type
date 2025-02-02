use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128770247: FileFormat = FileFormat {
    id: 128_770_247,
    source_type: SourceType::Wikidata,
    name: "CFEngine3 file format",
    extensions: &["cf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
