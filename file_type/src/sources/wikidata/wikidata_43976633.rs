use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43976633: FileFormat = FileFormat {
    id: 43_976_633,
    source_type: SourceType::Wikidata,
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
