use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126951686: FileFormat = FileFormat {
    id: 126_951_686,
    source_type: SourceType::Wikidata,
    name: "Nemerle source code file",
    extensions: &["n"],
    media_types: &["text/x-nemerle"],
    signatures: &[],
    related_formats: &[],
};
