use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206795: FileFormat = FileFormat {
    id: 28_206_795,
    source_type: SourceType::Wikidata,
    name: "OS/2 Boot Logo",
    extensions: &["lgo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
