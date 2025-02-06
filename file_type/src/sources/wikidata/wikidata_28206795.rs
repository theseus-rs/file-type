use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206795: FileFormat = FileFormat {
    id: 28_206_795,
    source_type: SourceType::Wikidata,
    name: "OS/2 Boot Logo",
    extensions: &["lgo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
