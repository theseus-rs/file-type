use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951804: FileFormat = FileFormat {
    id: 126_951_804,
    source_type: SourceType::Wikidata,
    name: "Prolog source code file",
    extensions: &["ecl", "pl", "pro", "prolog"],
    media_types: &["text/x-prolog"],
    internal_signatures: &[],
    related_formats: &[],
};
